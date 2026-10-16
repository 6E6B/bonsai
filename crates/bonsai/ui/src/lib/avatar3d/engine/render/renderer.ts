import * as THREE from 'three';
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls.js';
import { deg, rad } from '../misc/misc';
import { getRenderDescForInstance, type RenderDesc } from './renderDesc';
import { ObjectDesc } from './mainDescs/objectDesc';
import { CFrame, type Instance } from '../rblx/rbx';
import { type Authentication, createContentMap } from '../api';
import { FLAGS } from '../misc/flags';
import type { Vec3 } from '../mesh/mesh';
import { loadCompositMeshes } from './textureComposer';
import { setupWorkerPool } from '../misc/worker-pool';
import { RegisterWrappers } from '../rblx/wrapper-register';
import { error, log, warn } from '../misc/logger';
import { RegisterRenderDescs } from './mainDescs/renderDesc-register';
import { RBXRendererScene } from './rendererScene';

export function disposeMesh(scene: THREE.Scene, mesh: THREE.Mesh) {
    if (mesh.material) {
        const materials = Array.isArray(mesh.material) ? mesh.material : [mesh.material];

        for (const material of materials) {
            for (const key of Object.keys(material)) {
                const value = (material as unknown as {[K in string]: unknown})[key]
                if (value instanceof THREE.Texture) {
                    value.dispose()
                }
            }

            if (material instanceof THREE.ShaderMaterial) {
                const uniforms = material.uniforms
                for (const key of Object.keys(uniforms)) {
                    const value = uniforms[key].value
                    if (value instanceof THREE.Texture) {
                        value.dispose()
                    }
                }
            }
            
            material.dispose()
        }
    }
    if (mesh.geometry) {
        mesh.geometry.dispose()
    }
    scene.remove(mesh)
}

/**
 * A singleton, intialized by calling RBXRenderer.fullSetup()
 * 
 * @category Renderer
 */
export class RBXRenderer {
    static orbitControlsTarget: Vec3 = [0,3,0]

    static scenes: RBXRendererScene[] = [new RBXRendererScene()]
    
    static get firstScene() {
        return RBXRenderer.scenes[0]
    }

    /**@deprecated This can only get the first renderScene's scene */
    static get scene() {
        return RBXRenderer.firstScene.scene
    }
    /**@deprecated This can only get the first renderScene's camera */
    static get camera() {
        return RBXRenderer.firstScene.camera
    }
    /**@deprecated This can only get the first renderScene's controls */
    static get controls() {
        return RBXRenderer.firstScene.controls
    }

    static renderer?: THREE.WebGLRenderer

    static resolution: [number,number] = [420, 420]

    static backgroundColor: THREE.ColorRepresentation = 0x2b2d33
    /**@deprecated Use backgroundColor instead */
    static get backgroundColorHex() {return new THREE.Color(RBXRenderer.backgroundColor).getHex()}
    /**@deprecated Use backgroundColor instead */
    static set backgroundColorHex(val: number) {RBXRenderer.backgroundColor = val}
    static backgroundTransparent: boolean = false

    static canvasContainer: HTMLDivElement


    /**@deprecated This can only get the first renderScene's plane */
    static get plane() {return RBXRenderer.firstScene.plane}
    /**@deprecated This can only get the first renderScene's shadowPlane */
    static get shadowPlane() {return RBXRenderer.firstScene.shadowPlane}
    /**@deprecated This can only get the first renderScene's ambientLight */
    static get ambientLight() {return RBXRenderer.firstScene.ambientLight}
    /**@deprecated This can only get the first renderScene's directionalLight */
    static get directionalLight() {return RBXRenderer.firstScene.directionalLight}
    /**@deprecated This can only get the first renderScene's directionalLight2 */
    static get directionalLight2() {return RBXRenderer.firstScene.directionalLight2}
    /**@deprecated This can only set the first renderScene's wellLitDirectionalLightIntensity*/
    static set wellLitDirectionalLightIntensity(v: number) {
        RBXRenderer.firstScene.wellLitDirectionalLightIntensity = v
    }
    /**@deprecated This can only gey the first renderScene's wellLitDirectionalLightIntensity*/
    static get wellLitDirectionalLightIntensity() {
        return RBXRenderer.firstScene.wellLitDirectionalLightIntensity
    }

    static failedToCreate: boolean = false
    static error?: unknown

    static async boilerplateSetup() {
        RegisterRenderDescs()
        RegisterWrappers()
        createContentMap()
        setupWorkerPool()
        loadCompositMeshes()
    }

    static addScene(): RBXRendererScene {
        const renderScene = new RBXRendererScene()
        RBXRenderer.scenes.push(renderScene)
        return renderScene
    }

    /**Fully sets up renderer with scene, camera and frame rendering
     * @returns success
    */
    static async fullSetup(includeSceneAppearance: boolean = true, includeControls: boolean = true, includeAnimate: boolean = true): Promise<boolean> {
        try {
            RBXRenderer.createContainer()
            await RBXRenderer.boilerplateSetup()
            RBXRenderer.create()
            if (includeSceneAppearance) RBXRenderer.setupScene()
            if (includeControls) RBXRenderer.setupControls()
            if (includeAnimate) RBXRenderer.animateAll()
        } catch (err) {
            error(err)
            RBXRenderer.failedToCreate = true
            RBXRenderer.error = err
        }

        return !RBXRenderer.failedToCreate
    }

    /**Creates canvasContainer */
    static createContainer() {
        //create container
        RBXRenderer.canvasContainer = document.createElement("div")
        RBXRenderer.canvasContainer.style.position = "relative"
        RBXRenderer.canvasContainer.style.width = `${RBXRenderer.resolution[0]}px`
        RBXRenderer.canvasContainer.style.height = `${RBXRenderer.resolution[1]}px`
    }

    /**Sets up the THREE.js renderer */
    static create(canvas?: HTMLCanvasElement) {
        //create renderer
        RBXRenderer.renderer = new THREE.WebGLRenderer({antialias: true, alpha: true, premultipliedAlpha: true, canvas})
        if (RBXRenderer.renderer) RBXRenderer.renderer.setClearColor(new THREE.Color(0,0,0), 0);

        RBXRenderer.renderer.outputColorSpace = THREE.SRGBColorSpace
        RBXRenderer.renderer.shadowMap.enabled = true
        RBXRenderer.renderer.shadowMap.type = THREE.PCFShadowMap

        RBXRenderer.renderer.setPixelRatio(globalThis.devicePixelRatio * 1 || 1)
        RBXRenderer.renderer.setSize(...RBXRenderer.resolution);

        //if (FLAGS.USE_POST_PROCESSING && FLAGS.POST_PROCESSING_IS_DOUBLE_SIZE) {
        //    RBXRenderer.renderer.setSize(RBXRenderer.resolution[0] * 2, RBXRenderer.resolution[1] * 2)
        //}

        RBXRenderer.renderer.domElement.setAttribute("id","OutfitInfo-outfit-image-3d")

        //add renderDom
        RBXRenderer.canvasContainer.appendChild(RBXRenderer.renderer.domElement)

        RBXRenderer.setupLostContextHandler()
    }

    static setupLostContextHandler() {
        if (!RBXRenderer.renderer) return
        RBXRenderer.renderer.domElement.addEventListener("webglcontextlost", (e) => {
            e.preventDefault()
            error(true, "RBXRenderer's WebGL2 context was lost, consider optimizing your WebGL usage")
            if (FLAGS.AUTO_RESTORE_CONTEXT) {
                //create new canvas
                const newCanvas = document.createElement("canvas")
                if (RBXRenderer.renderer?.domElement) {
                    RBXRenderer.canvasContainer.replaceChild(newCanvas, RBXRenderer.renderer.domElement)
                }

                //recreate renderer
                RBXRenderer.renderer?.dispose()
                RBXRenderer.create(newCanvas)

                //update renderers
                for (const renderScene of RBXRenderer.scenes) {
                    //restore controls
                    const controls = renderScene.controls
                    if (controls) {
                        controls.dispose()
                        controls.domElement = newCanvas
                        controls.connect(newCanvas)
                    }

                    //mark rendertarget instances as dirty
                    for (const renderDesc of renderScene.renderDescs.values()) {
                        if (renderDesc instanceof ObjectDesc) {
                            const materialDesc = renderDesc.materialDesc
                            const composeType = materialDesc.getComposeType("color")

                            if (composeType === "full" || composeType === "simple") {
                                materialDesc.dirty = true
                            }
                        }
                    }
                }
            }
        })

        /*RBXRenderer.renderer.domElement.addEventListener("webglcontextrestored", () => {
            log(true, "RBXRenderer's WebGL2 context is being restored...")

            for (const renderScene of RBXRenderer.scenes) {
                const threeScene = renderScene.scene

                threeScene.traverse((obj) => {
                    if (obj instanceof THREE.Mesh) {
                        obj.geometry.attributes.position.needsUpdate = true
                        if (obj.material.map) obj.material.map.needsUpdate = true
                        obj.material.needsUpdate = true
                    }
                })
            }
        })*/
    }

    /**Sets up a basic scene with lighting
     * @param lightingType "WellLit" is the default lighting for RoAvatar, "Thumbnail" tries to match the Roblox thumbnail lighting
     * @param backgroundColor is the color code for the background color, for example 0x2b2d33
    */
    static setupScene(lightingType: "WellLit" | "Thumbnail" = "WellLit", backgroundColorVal = RBXRenderer.backgroundColor, renderScene: RBXRendererScene = RBXRenderer.firstScene) {
        //const backgroundColor = new THREE.Color( 0x2C2E31 )
        //const backgroundColor = new THREE.Color( 0x191a1f )
        //const backgroundColor = new THREE.Color( 0x2a2a2d )
        RBXRenderer.backgroundColor = backgroundColorVal

        const backgroundColor = new THREE.Color( backgroundColorVal )
        renderScene.scene.background = backgroundColor;

        const thumbnailAmbientVal = 128 //138 SHOULD be accurate but its not???, nvm it probably is but there is a second light source, wait i think ambient is more correct to use
        //thumbnailAmbientVal = 153 //this is 255 * 0.6
        let ambientLightColor = undefined
        if (lightingType === "Thumbnail") {
            ambientLightColor = new THREE.Color(thumbnailAmbientVal / 255, thumbnailAmbientVal / 255, thumbnailAmbientVal / 255)
        } else if (lightingType === "WellLit") {
            ambientLightColor = new THREE.Color(100 / 255, 100 / 255, 100 / 255)
        }
        //const ambientLight = new THREE.AmbientLight( 0x7a7a7a );
        const ambientLight = new THREE.AmbientLight( ambientLightColor, Math.PI / 2 );
        renderScene.scene.add( ambientLight );
        renderScene.ambientLight = ambientLight

        let directionalLightColor = undefined
        const directionalLightVal = Math.PI
        if (lightingType === "Thumbnail") {
            directionalLightColor = new THREE.Color(directionalLightVal, directionalLightVal, directionalLightVal)
        } else if (lightingType === "WellLit") {
            directionalLightColor = new THREE.Color(1,1,1)
        }
        let directionalLightIntensity = 1
        if (lightingType === "WellLit") {
            directionalLightIntensity = renderScene._wellLitDirectionalLightIntensity
        }

        const directionalLight = new THREE.DirectionalLight( directionalLightColor, directionalLightIntensity );
        //directionalLight.position.set(new THREE.Vector3(1.2,1,1.2))
        if (lightingType === "WellLit") {
            directionalLight.position.set(-5,15,-8)
        } else if (lightingType === "Thumbnail") {
            directionalLight.position.set(-0.47489210963249207 * 10, 0.8225368857383728 * 10, 0.3129066228866577 * 10)
        }

        if (lightingType === "WellLit") {
            directionalLight.castShadow = renderScene.shadowEnabled
        }
        directionalLight.shadow.mapSize.width = renderScene.shadowResolution[0];
        directionalLight.shadow.mapSize.height = renderScene.shadowResolution[1];

        const bottomOffset = 1.6
        const shadowPhysicalSize = 5
        directionalLight.shadow.camera.left = -shadowPhysicalSize
        directionalLight.shadow.camera.right = shadowPhysicalSize
        directionalLight.shadow.camera.top = shadowPhysicalSize + bottomOffset
        directionalLight.shadow.camera.bottom = -shadowPhysicalSize + bottomOffset

        directionalLight.shadow.camera.near = 0.5; // default
        directionalLight.shadow.camera.far = 25;

        if (lightingType === "WellLit") directionalLight.shadow.intensity = 0.5
        if (lightingType === "Thumbnail") {
            directionalLight.shadow.radius = 4
            directionalLight.shadow.bias = 0.0005
        }

        //const shadowHelper = new THREE.CameraHelper(directionalLight.shadow.camera);
        //scene.add(shadowHelper);

        directionalLight.target.position.set(0,0,0)
        renderScene.scene.add( directionalLight );
        renderScene.directionalLight = directionalLight

        if (lightingType === "WellLit") {
            const directionalLight2 = new THREE.DirectionalLight( 0xffffff, 0.3 );
            //directionalLight.position.set(new THREE.Vector3(1.2,1,1.2))
            directionalLight2.position.set(5,-7,5)
            directionalLight2.target.position.set(0,0,0)
            renderScene.scene.add( directionalLight2 );
            renderScene.directionalLight2 = directionalLight2
        } else if (lightingType === "Thumbnail") { //this looks good TODO: disable specular from this light somehow, should exclusively be diffuse
            const directionalLight2 = new THREE.DirectionalLight( directionalLightColor, directionalLightVal * 0.1 );
            //directionalLight.position.set(new THREE.Vector3(1.2,1,1.2))
            directionalLight2.position.set(-0.47489210963249207 * -10, 0.8225368857383728 * -10, 0.3129066228866577 * -10)
            directionalLight2.target.position.set(0,0,0)
            renderScene.scene.add( directionalLight2 );
            renderScene.directionalLight2 = directionalLight2
        }

        const planeGeometry = new THREE.PlaneGeometry( 100, 100, 1, 1 );
        const planeShadowMaterial = new THREE.ShadowMaterial({opacity: 1.0});
        const shadowPlane = new THREE.Mesh( planeGeometry, planeShadowMaterial );
        shadowPlane.rotation.set(rad(-90),0,0)
        shadowPlane.position.set(0,0,0)
        shadowPlane.receiveShadow = true;
        renderScene.shadowPlane = shadowPlane
        renderScene.scene.add( shadowPlane );

        const planeSolidColorMaterial = new THREE.MeshBasicMaterial({color: backgroundColor})
        const plane = new THREE.Mesh( planeGeometry, planeSolidColorMaterial );
        plane.rotation.set(rad(-90),0,0)
        plane.position.set(0,0,0)
        plane.receiveShadow = false;
        plane.renderOrder = -3
        renderScene.plane = plane
        renderScene.scene.add( plane );
    }

    /**Sets up orbit controls */
    static setupControls(renderScene: RBXRendererScene = RBXRenderer.firstScene) {
        if (!RBXRenderer.renderer) return
        //orbit controls
        const controls = new OrbitControls(renderScene.camera, RBXRenderer.renderer.domElement)
        controls.maxDistance = 25
        controls.zoomSpeed = 2

        controls.target.set(...RBXRenderer.orbitControlsTarget)
        log(false, controls.target)

        renderScene.controls = controls
        renderScene.camera.position.set(renderScene.lookAwayVector[0] * renderScene.lookAwayDistance,3 + renderScene.lookAwayVector[1] * renderScene.lookAwayDistance,renderScene.lookAwayVector[2] * renderScene.lookAwayDistance)
        renderScene.camera.lookAt(new THREE.Vector3(...RBXRenderer.orbitControlsTarget))
        controls.update()
    }

    /**
     * @param color example: 0x2b2d33 which is the default
     */
    static setBackgroundColor(color: THREE.ColorRepresentation) {
        RBXRenderer.backgroundColor = color
        for (const renderScene of RBXRenderer.scenes) {
            if (RBXRenderer.backgroundTransparent) {
                renderScene.scene.background = null
            } else {
                renderScene.scene.background = new THREE.Color( RBXRenderer.backgroundColor )
            }
            if (renderScene.plane) {
                renderScene.plane.visible = !RBXRenderer.backgroundTransparent
            }
            if (renderScene.plane) {
                renderScene.plane.material = new THREE.MeshBasicMaterial({color: color})
            }
        }
    }

    /**
     * Sets the background and ground's visibility, but not the shadowPlane
     * @param transparent If the background and ground should be transparent
     */
    static setBackgroundTransparent(transparent: boolean) {
        RBXRenderer.backgroundTransparent = transparent
        if (RBXRenderer.renderer) RBXRenderer.renderer.setClearColor(new THREE.Color(0,0,0), 0);
        for (const renderScene of RBXRenderer.scenes) {
            if (RBXRenderer.backgroundTransparent) {
                renderScene.scene.background = null
            } else {
                renderScene.scene.background = new THREE.Color( RBXRenderer.backgroundColor )
            }
            if (renderScene.plane) {
                renderScene.plane.visible = !RBXRenderer.backgroundTransparent
            }
        }
    }

    /**@deprecated Makes the renderer render a new frame on every animationFrame */
    static animate(shouldRequestAnimationFrame: boolean = true) {
        if (!RBXRenderer.renderer) return
        RBXRenderer.renderScene(RBXRenderer.firstScene)

        if (shouldRequestAnimationFrame) {
            requestAnimationFrame( () => {
                RBXRenderer.animate()
            } );
        }
    }

    static animateAll(shouldRequestAnimationFrame: boolean = true) {
        if (!RBXRenderer.renderer) return
        //a frame that throws must not end the loop
        try {
            for (const renderScene of RBXRenderer.scenes) {
                RBXRenderer.renderScene(renderScene, renderScene === RBXRenderer.firstScene)
            }
        } catch (e) {
            console.warn("[3d] frame failed:", e)
        }

        if (shouldRequestAnimationFrame) {
            requestAnimationFrame(() => {
                RBXRenderer.animateAll()
            })
        }
    }

    static renderScene(renderScene: RBXRendererScene, autoClear: boolean = true) {
        if (!RBXRenderer.renderer) return
        if (!renderScene.shouldAnimate) return
        RBXRenderer.renderer.autoClear = autoClear
        if (!autoClear) {
            RBXRenderer.renderer.clearDepth()
        }
        RBXRenderer.renderer.setRenderTarget(null)

        //fix viewport and scissor
        let [x, y] = [0,0]
        let [width, height] = RBXRenderer.resolution
        if (renderScene.viewport) {
            x = renderScene.viewport[0]
            y = renderScene.viewport[1]
            width = renderScene.viewport[2]
            height = renderScene.viewport[3]
        }

        RBXRenderer.renderer.setViewport(x, y, width, height)

        if (renderScene.scissor) {
            RBXRenderer.renderer.setScissorTest(true)
            RBXRenderer.renderer.setScissor(...renderScene.scissor)
        } else {
            RBXRenderer.renderer.setScissorTest(false)
        }

        renderScene.camera.aspect = width / height
        renderScene.camera.updateProjectionMatrix()

        //actually render
        if (width > 0 && height > 0) {
            RBXRenderer.renderer.render(renderScene.scene, renderScene.camera)
        }

        RBXRenderer.renderer.autoClear = true
    }

    /**Removes an instance from the renderer */
    static removeInstance(instance: Instance, renderScene: RBXRendererScene = RBXRenderer.firstScene) {
        if (!RBXRenderer.renderer) return
        //console.log("Removed instance:", instance.Prop("Name"), instance.id)

        const index = renderScene.addedInstances.indexOf(instance)
        if (index >= 0) {
            renderScene.addedInstances[index] = renderScene.addedInstances[renderScene.addedInstances.length - 1]
            renderScene.addedInstances.splice(renderScene.addedInstances.length - 1, 1)
        }

        const desc = renderScene.renderDescs.get(instance)
        if (desc) {
            desc.dispose(RBXRenderer.renderer, renderScene.scene)
        }

        renderScene.renderDescs.delete(instance)
        renderScene.isRenderingMesh.delete(instance)

        renderScene.compiledRenderDesc.Fire(instance)

        for (const child of instance.GetChildren()) {
            RBXRenderer.removeInstance(child, renderScene)
        }
    }

    static getRenderDescs(instance: Instance): RenderDesc[] {
        const renderDescs: RenderDesc[] = []

        for (const renderScene of RBXRenderer.scenes) {
            const renderDesc = renderScene.renderDescs.get(instance)
            if (renderDesc) renderDescs.push(renderDesc)
        }

        return renderDescs
    }

    private static _addSkeletonToResult(result: THREE.Object3D, newDesc: RenderDesc, renderScene: RBXRendererScene) {
        //update skeletonDesc for RenderDescs that have that
        if (result instanceof THREE.SkinnedMesh && newDesc instanceof ObjectDesc) {
            const skeleton = newDesc.skeletonDesc?.skeleton
            
            if (skeleton) {
                result.bindMode = "detached"
                if (newDesc.skeletonDesc) {
                    if (FLAGS.USE_LOCAL_SKELETONDESC) {
                        result.add(newDesc.skeletonDesc.rootBone)
                    } else {
                        renderScene.scene.add(newDesc.skeletonDesc.rootBone)
                    }
                }
                result.bind(skeleton)
                renderScene.scene.add(result)
            }
        } else {
            renderScene.scene.add(result)
        }
    }

    private static _updateCompiledRenderDesc(oldDesc: RenderDesc, newDesc: RenderDesc, renderScene: RBXRendererScene, instance: Instance) {
        //fire compiled event to scene
        if (!oldDesc.compiled && !oldDesc.failed) {
            oldDesc.compiled = true
            renderScene.compiledRenderDesc.Fire(instance)
        }

        //do nothing except update
        if (!oldDesc.isSame(newDesc)) {
            oldDesc.fromRenderDesc(newDesc)
            oldDesc.updateResults()
        }
    }

    private static _addRenderDesc(instance: Instance, auth: Authentication, DescClass: typeof RenderDesc, renderScene: RBXRendererScene): void {
        if (!RBXRenderer.renderer) return
        const oldDesc = renderScene.renderDescs.get(instance)
        const newDesc = new DescClass(renderScene)
        newDesc.fromInstance(instance)

        if (oldDesc && !oldDesc.needsRegeneration(newDesc) && !renderScene.isRenderingMesh.get(instance)) {
            RBXRenderer._updateCompiledRenderDesc(oldDesc, newDesc, renderScene, instance)
        } else {
            //generate new mesh
            if (!renderScene.isRenderingMesh.get(instance)) {
                //console.log(`Generating ${instance.Prop("Name")} ${instance.id}`)

                if (oldDesc && Object.getPrototypeOf(oldDesc).constructor === Object.getPrototypeOf(newDesc).constructor) {
                    newDesc.transferFrom(oldDesc)
                    //newDesc.results = oldDesc.results //this is done so that the result can be disposed if removeInstance is called during generation, LATER COMMENT: why is it done like this, why dont we dispose oldDesc no matter what
                }
                renderScene.renderDescs.set(instance, newDesc)
                renderScene.isRenderingMesh.set(instance, true)

                //get the mesh
                newDesc.compileResults(RBXRenderer.renderer, renderScene.scene).then(results => {
                    if (RBXRenderer.renderer) oldDesc?.dispose(RBXRenderer.renderer, renderScene.scene)

                    if (results && !(results instanceof Response)) {
                        //sucessfully compiled
                        newDesc.updateResults()

                        if (renderScene.renderDescs.get(instance) === newDesc && RBXRenderer.renderer) {
                            for (const result of results) {
                                RBXRenderer._addSkeletonToResult(result, newDesc, renderScene)
                            }

                            //console.log(`Generated ${instance.Prop("Name")} ${instance.id}`)
                            renderScene.isRenderingMesh.set(instance, false)
                            RBXRenderer.addInstance(instance, auth, renderScene) //check instance again in case it changed during compilation
                        } else if (RBXRenderer.renderer) {
                            newDesc.dispose(RBXRenderer.renderer, renderScene.scene)
                        }
                    } else {
                        //failed to compile
                        if (RBXRenderer.renderer) newDesc.dispose(RBXRenderer.renderer, renderScene.scene)
                        newDesc.failed = true
                        if (renderScene.renderDescs.get(instance) === newDesc) {
                            renderScene.isRenderingMesh.set(instance, false)
                            renderScene.failedRenderDesc.Fire(instance)
                        }
                        warn(false, "Failed to compile mesh", this, results)
                    }
                })
            }
        }

        //destroy connection for instance
        if (!renderScene.destroyConnections.get(instance)) {
            renderScene.destroyConnections.set(instance, instance.Destroying.Connect(() => {
                RBXRenderer.removeInstance(instance, renderScene)
                const connection = renderScene.destroyConnections.get(instance)
                connection?.Disconnect()
                renderScene.destroyConnections.delete(instance)
            }))
        }
    }

    /** */
    /**
     * Adds an instance to the renderer or updates it, add it again to update it after changes have been made
     * @param instance The instance that will be updated (the descendants will also update)
     * @param auth This is here just in case something requires it in the future
     * @param renderScene The scene the instances will be rendered in. Default RBXRenderer.firstScene
     * @param includeDescendants If descendants should also have addInstance called on them. Default true
     * @returns 
     */
    static addInstance(instance: Instance, auth: Authentication, renderScene: RBXRendererScene = RBXRenderer.firstScene, includeDescendants: boolean = true) {
        if (renderScene.destroyed) return

        const RenderDescType = getRenderDescForInstance(instance)
        if (RenderDescType) {
            if (renderScene.addedInstances.indexOf(instance) < 0) renderScene.addedInstances.push(instance)
            RBXRenderer._addRenderDesc(instance, auth, RenderDescType, renderScene)
        }

        //update descendants  too
        if (includeDescendants) {
            for (const child of instance.GetDescendants()) {
                RBXRenderer.addInstance(child, auth, renderScene, false)
            }
        }
    }

    static setRendererSize(width: number, height: number): void {
        if (!RBXRenderer.renderer) return
        RBXRenderer.resolution = [width, height]
        RBXRenderer.renderer.domElement.setAttribute("style",`width: ${RBXRenderer.resolution[0]}px; height: ${RBXRenderer.resolution[1]}px; border-radius: 0px;`)
        RBXRenderer.canvasContainer.style.width = `${RBXRenderer.resolution[0]}px`
        RBXRenderer.canvasContainer.style.height = `${RBXRenderer.resolution[1]}px`
        RBXRenderer.renderer.setSize(width, height)
    }

    /**
     * @deprecated Use getRendererElement instead which includes the loading icon
     * @returns The element for the renderer canvas
     */
    static getRendererDom(): HTMLCanvasElement | undefined {
        if (!RBXRenderer.renderer) return
        return RBXRenderer.renderer.domElement
    }

    /**
     * @returns An element containing the renderer canvas
     */
    static getRendererElement(): HTMLDivElement {
        return RBXRenderer.canvasContainer
    }

    /**@deprecated This can only get the first renderScene's camera */
    static getRendererCamera(): THREE.PerspectiveCamera {
        return RBXRenderer.camera
    }

    static getCameraCFrame(renderScene: RBXRendererScene = RBXRenderer.firstScene): CFrame {
        const camera = renderScene.camera
        const pos = camera.position
        let rot = camera.rotation.clone()
        rot = rot.reorder("YXZ")

        const cf = new CFrame()
        cf.Position = pos.toArray()
        cf.Orientation = [deg(rot.x), deg(rot.y), deg(rot.z)]
        return cf
    }

    static setCameraCFrame(cameraCF: CFrame, renderScene: RBXRendererScene = RBXRenderer.firstScene): void {
        const camPos = new THREE.Vector3()
        const camQuat = new THREE.Quaternion()
        const camScale = new THREE.Vector3()

        const camMatrix = cameraCF.getTHREEMatrix()
        camMatrix.decompose(camPos, camQuat, camScale)

        renderScene.camera.position.set(...camPos.toArray())
        renderScene.camera.quaternion.set(...camQuat.toArray())

        renderScene.camera.updateMatrixWorld()
    }

    static setCameraFov(fov: number, renderScene: RBXRendererScene = RBXRenderer.firstScene) {
        renderScene.camera.fov = fov
    }

    /**@deprecated This can only get the first renderScene's controls */
    static getRendererControls(): OrbitControls | undefined {
        return RBXRenderer.controls
    }

    /**
     * @returns ThreeJS renderer
     */
    static getRenderer(): THREE.WebGLRenderer | undefined {
        return RBXRenderer.renderer
    }

    /**@deprecated This can only get the first renderScene's scene */
    static getScene() {
        return RBXRenderer.scene
    }
}

/**
 * @deprecated Use mountElement instead
 * @param container 
 * @category Renderer
 */
export function mount( container: HTMLDivElement ) {
    if (!RBXRenderer.renderer) return
    if (container) {
        container.insertBefore(RBXRenderer.renderer.domElement, container.firstChild)
    } else {
        RBXRenderer.renderer.domElement.remove()
    }
}

/**
 * Mounting function for use with React
 * @param container Container to mount inside
 * @category Renderer
 * @example
 * ```ts
 * export default function Component(): React.JSX.Element {
 *     const containerRef = useCallback(mountElement, [])
 * 
 *     return <div ref={containerRef}></div>
 * }
 * ```
 */
export function mountElement( container: HTMLDivElement ) {
    if (container) {
        container.insertBefore(RBXRenderer.canvasContainer, container.firstChild)
    } else {
        RBXRenderer.canvasContainer.remove()
    }
}