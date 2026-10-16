import * as THREE from "three"
import type { OrbitControls } from 'three/examples/jsm/controls/OrbitControls.js';
import { type Connection, type Instance, Event } from '../rblx/rbx';
import type { Vec3 } from '../mesh/mesh';
import type { RenderDesc } from "./renderDesc";
import { FLAGS } from "../misc/flags";
import { disposeMesh, RBXRenderer } from "./renderer";

/**
 * Created by calling RBXRenderer.addScene()
 * @category Renderer
 */
export class RBXRendererScene {
    //important scene components
    scene: THREE.Scene = new THREE.Scene()
    camera: THREE.PerspectiveCamera = new THREE.PerspectiveCamera( 70, 1 / 1, 0.1, 1000 )
    controls: OrbitControls | undefined

    shouldAnimate: boolean = true
    destroyed: boolean = false

    /**Used internally to determine color space output */
    isForRenderTarget: boolean = false

    get colorSpace(): THREE.ColorSpace {
        return this.isForRenderTarget ? "srgb-linear" : "srgb"
    }

    //viewport
    scissor?: [number, number, number, number]
    viewport?: [number, number, number, number]

    //renderables data
    addedInstances: Instance[] = []
    isRenderingMesh: Map<Instance,boolean> = new Map()
    renderDescs: Map<Instance,RenderDesc> = new Map()
    destroyConnections: Map<Instance,Connection> = new Map()
    
    compiledRenderDesc: Event = new Event()
    failedRenderDesc: Event = new Event()

    forceAccurateNeedsRegeneration: boolean = false
    particlesStartFull?: number = undefined
    particlesStartFullFramerate: number = FLAGS.PARTICLES_START_FULL_FRAMERATE

    //scene appearance config
    lookAwayVector: Vec3 = [0.406, 0.306, -0.819]
    lookAwayDistance: number = 6

    shadowEnabled: boolean = true
    shadowResolution: [number, number] = [256, 256]
    _wellLitDirectionalLightIntensity: number = Math.PI / 2

    set wellLitDirectionalLightIntensity(v: number) {
        this._wellLitDirectionalLightIntensity = v
        if (this.directionalLight) {
            this.directionalLight.intensity = this._wellLitDirectionalLightIntensity
        }
    }

    get wellLitDirectionalLightIntensity() {
        return this._wellLitDirectionalLightIntensity
    }

    //scene appearance
    plane?: THREE.Mesh
    shadowPlane?: THREE.Mesh
    ambientLight?: THREE.AmbientLight
    directionalLight?: THREE.DirectionalLight
    directionalLight2?: THREE.DirectionalLight

    /** Forces viewport to be within bounds */
    setRect(bounds: DOMRect) {
        this.viewport = [bounds.left, window.innerHeight - bounds.bottom, bounds.width, bounds.height]
        this.scissor = [...this.viewport]
    }

    /** Makes viewport size 0x0, invisible */
    noRect() {
        this.viewport = [0,0,0,0]
        this.scissor = [0,0,0,0]
    }

    /** Destroys all renderDescs but does not call Destroy on instances */
    destroy() {
        if (this.destroyed) return
        this.destroyed = true

        for (const instance of this.renderDescs.keys()) {
            RBXRenderer.removeInstance(instance, this)
        }

        RBXRenderer.scenes.splice(RBXRenderer.scenes.indexOf(this), 1)

        if (this.plane) {
            disposeMesh(this.scene, this.plane)
            this.plane = undefined
        }
        if (this.shadowPlane) {
            disposeMesh(this.scene, this.shadowPlane)
            this.shadowPlane = undefined
        }
    }

    /**
     * Checks that renderDescs for provided instances are compiled, ignores instances that shouldn't be compiled or haven't been added
     * @returns true if all provided instances's renderDescs have been compiled
     */
    areInstancesCompiled(instances: Instance[]): boolean {
        for (const instance of instances) {
            if (!this.addedInstances.includes(instance)) continue;

            const renderDesc = this.renderDescs.get(instance)
            if (!renderDesc || !renderDesc.compiled || this.isRenderingMesh.get(instance)) {
                return false
            }
        }

        return true
    }

    /**
     * Checks that all renderDescs are compiled
     * @return true if all have been compiled
     */
    isFullyCompiled(): boolean {
        return this.areInstancesCompiled(this.addedInstances)
    }

    /**
     * @param instances Array of instances you want to wait to be finished compiling
     * @param timeoutSeconds Default is 30 seconds
     * @returns true if successful, false if a RenderDesc failed to compile or timed out
     */
    async waitUntilFullyCompiled(instances: Instance[], timeoutSeconds: number = 30): Promise<boolean> {
        if (this.areInstancesCompiled(instances)) return true

        const connections: Connection[] = []

        const compiledSuccessfully = await Promise.race([
            //on fail compilation
            new Promise<boolean>((resolve) => {
                connections.push(this.failedRenderDesc.Connect(() => {
                    resolve(false)
                }))
            }),
            //on success compilation, check if everything is successfull
            new Promise<boolean>((resolve) => {
                connections.push(this.compiledRenderDesc.Connect(() => {
                    if (this.areInstancesCompiled(instances)) {
                        resolve(true)
                    }
                }))
            }),
            //on fail due to timeout
            new Promise<boolean>((resolve) => {setTimeout(() => {resolve(false)}, timeoutSeconds * 1000)})
        ])

        for (const connection of connections) {
            connection.Disconnect()
        }

        return compiledSuccessfully
    }
}