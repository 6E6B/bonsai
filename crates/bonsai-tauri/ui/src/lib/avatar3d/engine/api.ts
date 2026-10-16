// Trimmed for Bonsai: the upstream web API client is gone. Every asset fetch goes through
// `HOST.assetUrl`, which the app points at its `rbxasset://` protocol (rbx-api fetches and
// caches the bytes in Rust). Metadata lookups the renderer doesn't need fail with a Response.
import { FileMesh } from "./mesh/mesh"
import { Event, RBX } from "./rblx/rbx"
import { FLAGS } from "./misc/flags"
import type { BundleDetails_Result, ItemDetails_Result } from "./api-constant"
import type { Outfit } from "./avatar/outfit"
import { warn } from "./misc/logger"

/** Set by the app. Builds the URL that serves one asset's bytes. */
export const HOST = {
    assetUrl: (_id: number, _format?: string, _representations?: string): string | undefined => undefined,
}

/** Kept so engine signatures stay as upstream. Holds nothing; the app picks the account. */
export class Authentication {
    info?: {id: number, name: string, displayName: string}
}

function failed(status = 404) {
    return new Response(null, {status})
}

const currentlyLoadingAssets: string[] = []

export type CurrentlyLoadingUpdateType = "start" | "finish"

function _updateCurrentlyLoadingAssets(type: CurrentlyLoadingUpdateType, label: string) {
    API.Events.OnLoadingAssets.Fire(currentlyLoadingAssets.length > 0, type, label)
}

export class Cache<K, V> {
    map: Map<K,V> = new Map<K,V>()
    lastAccess: Map<K,number> = new Map()

    maxEntries: number
    minDeleteLifetime: number = 5

    onDelete?: (key: K) => void

    constructor(maxEntries: number = 250) {
        this.maxEntries = maxEntries
    }

    get(key: K): V | undefined {
        if (this.map.has(key)) {
            this.lastAccess.set(key, Date.now())
        }

        return this.map.get(key)
    }

    set(key: K, value: V): Cache<K,V> {
        this.map.set(key, value)
        this.lastAccess.set(key, Date.now())

        this.deleteUntil()

        return this
    }

    has(key: K): boolean {
        return this.map.has(key)
    }

    delete(key: K): boolean {
        if (this.onDelete) this.onDelete(key)

        const toReturn = this.map.delete(key)
        this.lastAccess.delete(key)

        return toReturn
    }

    deleteUntil() {
        while (this.map.size > this.maxEntries) {
            if (!this.deleteOne()) break
        }
    }

    deleteOne(): boolean {
        //delete from cache when it reaches limit
        if (this.map.size > this.maxEntries) {
            const toDelete = [...this.lastAccess.entries()].reduce((min, current) => {
                return current[1] < min[1] ? current : min
            })

            if (Date.now() - toDelete[1] >= this.minDeleteLifetime * 1000) {
                this.delete(toDelete[0])
                return true
            }
        }

        return false
    }
}

/**
 * @category API
 */

/** @category API */
export const CACHE = {
    "AssetBuffer": new Cache<string,Promise<Response | ArrayBuffer>>(100),
    "RBX": new Cache<string,RBX>(100),
    "Mesh": new Cache<string,FileMesh>(100),
    "Image": new Cache<string,Promise<HTMLImageElement | undefined> | HTMLImageElement | undefined>(100),
    "IsLayered": new Cache<number,boolean>(1000),
}

export const ContentMap = new Map<string,string>()

export function createContentMap() {
    ContentMap.set("rbxasset://fonts/BaseballCap.mesh",	"12220916")
    ContentMap.set("rbxasset://fonts/clonewand.mesh", "12221344")
    ContentMap.set("rbxasset://fonts/fusedgirl.mesh", "12221423")
    ContentMap.set("rbxasset://fonts/girlhair.mesh", "12221431")
    ContentMap.set("rbxasset://fonts/hammer.mesh", "12221451")
    ContentMap.set("rbxasset://fonts/NinjaMask.mesh", "12221524")
    ContentMap.set("rbxasset://fonts/paintballgun.mesh", "11900867")
    ContentMap.set("rbxasset://fonts/pawn.mesh", "12221585")
    ContentMap.set("rbxasset://fonts/PirateHat.mesh", "12221595")
    ContentMap.set("rbxasset://fonts/PoliceCap.mesh", "12221603")
    ContentMap.set("rbxasset://fonts/rocketlauncher.mesh", "12221651")
    ContentMap.set("rbxasset://fonts/slingshot.mesh", "12221682")
    ContentMap.set("rbxasset://fonts/sombrero.mesh", "12221705")
    ContentMap.set("rbxasset://fonts/sword.mesh", "12221720")
    ContentMap.set("rbxasset://fonts/timebomb.mesh", "12221733")
    ContentMap.set("rbxasset://fonts/tophat.mesh", "12221750")
    ContentMap.set("rbxasset://fonts/tree.mesh", "12221787")
    ContentMap.set("rbxasset://fonts/trowel.mesh", "12221793")
    ContentMap.set("rbxasset://fonts/VikingHelmet.mesh", "12221815")

    if (FLAGS.ONLINE_ASSETS) {
        //particles
        ContentMap.set("rbxasset://textures/particles/SquareParticle.png", "rbxassetid://81536466622192")
        ContentMap.set("rbxasset://textures/particles/sparkles_main.png", "rbxassetid://87394320952325")
        ContentMap.set("rbxasset://textures/particles/sparkles_color.png", "rbxassetid://133057519435872")
        ContentMap.set("rbxasset://textures/particles/smoke_main.png", "rbxassetid://138162637433023")
        ContentMap.set("rbxasset://textures/particles/smoke_color.png", "rbxassetid://125845160650254")
        ContentMap.set("rbxasset://textures/particles/legacy_fire_alpha_color.png", "rbxassetid://118480963109188")
        ContentMap.set("rbxasset://textures/particles/forcefield_vortex_main.png", "rbxassetid://90748587720186")
        ContentMap.set("rbxasset://textures/particles/forcefield_vortex_color.png", "rbxassetid://114134905691033")
        ContentMap.set("rbxasset://textures/particles/forcefield_glow_main.png", "rbxassetid://118476782259878")
        ContentMap.set("rbxasset://textures/particles/forcefield_glow_color.png", "rbxassetid://136830612119909")
        ContentMap.set("rbxasset://textures/particles/forcefield_glow_alpha.png", "rbxassetid://127202098262235")
        ContentMap.set("rbxasset://textures/particles/forcefield_alpha.png", "rbxassetid://114076943026381")
        ContentMap.set("rbxasset://textures/particles/fire_sparks_main.png", "rbxassetid://104223768023861")
        ContentMap.set("rbxasset://textures/particles/fire_sparks_color.png", "rbxassetid://140709297018478")
        ContentMap.set("rbxasset://textures/particles/fire_main.png", "rbxassetid://95195318391696")
        ContentMap.set("rbxasset://textures/particles/fire_color.png", "rbxassetid://71855354689630")
        ContentMap.set("rbxasset://textures/particles/fire_alpha.png", "rbxassetid://137123937216738")
        ContentMap.set("rbxasset://textures/particles/explosion01_smoke_main.png", "rbxassetid://93992972651499")
        ContentMap.set("rbxasset://textures/particles/explosion01_smoke_color_new.png", "rbxassetid://86144670293531")
        ContentMap.set("rbxasset://textures/particles/explosion01_smoke_alpha.png", "rbxassetid://138742967158809")
        ContentMap.set("rbxasset://textures/particles/explosion01_shockwave_main.png", "rbxassetid://116101962979677")
        ContentMap.set("rbxasset://textures/particles/explosion01_implosion_main.png", "rbxassetid://86764380638770")
        ContentMap.set("rbxasset://textures/particles/explosion01_implosion_color.png", "rbxassetid://127669378194473")
        ContentMap.set("rbxasset://textures/particles/explosion01_core_main.png", "rbxassetid://94075508535469")
        ContentMap.set("rbxasset://textures/particles/explosion01_core_alpha.png", "rbxassetid://99241903368204")
        ContentMap.set("rbxasset://textures/particles/explosion_color.png", "rbxassetid://93670753826336")
        ContentMap.set("rbxasset://textures/particles/explosion_alpha.png", "rbxassetid://115975840525503")
        ContentMap.set("rbxasset://textures/particles/common_alpha.png", "rbxassetid://76897028114182")

        //textures
        ContentMap.set("rbxasset://textures/face.png", "rbxassetid://126076136486265")

        //meshes
        ContentMap.set("rbxasset://avatar/meshes/torso.mesh", "85617550700848")
        ContentMap.set("rbxasset://avatar/meshes/rightleg.mesh", "72388106457735")
        ContentMap.set("rbxasset://avatar/meshes/rightarm.mesh", "111146997566389")
        ContentMap.set("rbxasset://avatar/meshes/leftleg.mesh", "90013209978879")
        ContentMap.set("rbxasset://avatar/meshes/leftarm.mesh", "107015122138989")
        ContentMap.set("rbxasset://avatar/heads/headP.mesh", "92893933522111")
        ContentMap.set("rbxasset://avatar/heads/headO.mesh", "77541016415293")
        ContentMap.set("rbxasset://avatar/heads/headN.mesh", "135830315772083")
        ContentMap.set("rbxasset://avatar/heads/headM.mesh", "72296270663395")
        ContentMap.set("rbxasset://avatar/heads/headL.mesh", "118173713712494")
        ContentMap.set("rbxasset://avatar/heads/headK.mesh", "87988019718389")
        ContentMap.set("rbxasset://avatar/heads/headJ.mesh", "87602015805248")
        ContentMap.set("rbxasset://avatar/heads/headI.mesh", "105758863945258")
        ContentMap.set("rbxasset://avatar/heads/headH.mesh", "82654813279081")
        ContentMap.set("rbxasset://avatar/heads/headG.mesh", "78217037620613")
        ContentMap.set("rbxasset://avatar/heads/headF.mesh", "81353062523004")
        ContentMap.set("rbxasset://avatar/heads/headE.mesh", "113298530836166")
        ContentMap.set("rbxasset://avatar/heads/headD.mesh", "84378174703962")
        ContentMap.set("rbxasset://avatar/heads/headC.mesh", "114955209749256")
        ContentMap.set("rbxasset://avatar/heads/headB.mesh", "86540893905615")
        ContentMap.set("rbxasset://avatar/heads/headA.mesh", "77314442147190")
        ContentMap.set("rbxasset://avatar/heads/head.mesh", "84927473172716")
        ContentMap.set("rbxasset://avatar/compositing/R15CompositTorsoBase.mesh", "128898737887110")
        ContentMap.set("rbxasset://avatar/compositing/R15CompositRightArmBase.mesh", "130403245999873")
        ContentMap.set("rbxasset://avatar/compositing/R15CompositLeftArmBase.mesh", "122438986243654")
        ContentMap.set("rbxasset://avatar/compositing/CompositTShirt.mesh", "73352601509228")
        ContentMap.set("rbxasset://avatar/compositing/CompositTorsoBase.mesh", "110815013780474")
        ContentMap.set("rbxasset://avatar/compositing/CompositShirtTemplate.mesh", "99918409598660")
        ContentMap.set("rbxasset://avatar/compositing/CompositRightLegBase.mesh", "107252470435894")
        ContentMap.set("rbxasset://avatar/compositing/CompositRightArmBase.mesh", "86285571969113")
        ContentMap.set("rbxasset://avatar/compositing/CompositQuad.mesh", "86988649991001")
        ContentMap.set("rbxasset://avatar/compositing/CompositPantsTemplate.mesh", "108749124518615")
        ContentMap.set("rbxasset://avatar/compositing/CompositLeftLegBase.mesh", "126843307192697")
        ContentMap.set("rbxasset://avatar/compositing/CompositLeftArmBase.mesh", "124757651657333")
        ContentMap.set("rbxasset://avatar/compositing/CompositFullAtlasOverlayTexture.mesh", "99832238544592")
        ContentMap.set("rbxasset://avatar/compositing/CompositFullAtlasBaseTexture.mesh", "82745085603378")
        ContentMap.set("rbxasset://avatar/compositing/CompositExtraSlot4.mesh", "90442767118712")
        ContentMap.set("rbxasset://avatar/compositing/CompositExtraSlot3.mesh", "114465125672717")
        ContentMap.set("rbxasset://avatar/compositing/CompositExtraSlot2.mesh", "111697299800137")
        ContentMap.set("rbxasset://avatar/compositing/CompositExtraSlot1.mesh", "92101687933174")
        ContentMap.set("rbxasset://avatar/compositing/CompositExtraSlot0.mesh", "115275146529093")
    }

    //rigs
    if (FLAGS.ONLINE_ASSETS) {
        //rbxm
        ContentMap.set("roavatar://RigR6.rbxm", "134202675113006")
        ContentMap.set("roavatar://RigR15.rbxm", "117612227055721")
    } else {
        //rbxm
        ContentMap.set("roavatar://RigR6.rbxm", FLAGS.RIG_PATH + "RigR6.rbxm")
        ContentMap.set("roavatar://RigR15.rbxm", FLAGS.RIG_PATH + "RigR15.rbxm")
    }

    //from roavatar, always online
    ContentMap.set("roavatar://AvatarEditorScene.rbxm", "74148511291027")
    ContentMap.set("roavatar://AvatarSceneNew.rbxm", "130507237273896")
    ContentMap.set("roavatar://AvatarCyclorama.rbxm", "79116945688799")
}

type Resolved = {url: string} | {id: number} | undefined

/** Where an asset string's bytes live: a bundled file or a Roblox asset id. */
function resolveAsset(str: string): Resolved {
    str = String(str)
    const mapped = ContentMap.get(str)
    if (mapped) str = mapped

    if (str.startsWith("rbxasset://")) {
        return {url: FLAGS.ASSETS_PATH + str.slice(11).replaceAll("\\", "/")}
    }
    if (str.startsWith("/") || str.startsWith(".")) {
        return {url: str}
    }
    if (/^\d+$/.test(str) || str.startsWith("rbxassetid://") || str.includes("roblox.com/asset") || str.includes("assetdelivery.roblox.com")) {
        const id = API.Misc.idFromStr(str)
        return Number.isSafeInteger(id) && id > 0 ? {id} : undefined
    }
    return undefined
}

function formatOf(headers?: HeadersInit): string | undefined {
    if (!headers) return undefined
    return new Headers(headers).get("Roblox-AssetFormat") || undefined
}

/** The URL to load, or undefined when the string names nothing we can serve (raw CDN URLs). */
function assetUrl(str: string, headers?: HeadersInit, representations?: string): string | undefined {
    const r = resolveAsset(str)
    if (!r) return undefined
    if ("url" in r) return r.url
    return HOST.assetUrl(r.id, formatOf(headers), representations)
}

async function getAssetBufferInternal(url: string, headers?: HeadersInit, representations?: string): Promise<Response | ArrayBuffer> {
    const fetchStr = assetUrl(url, headers, representations)
    if (!fetchStr) {
        warn(true, "Can't load asset", url)
        return failed()
    }
    const label = `asset-${fetchStr}`
    API.Misc.startCurrentlyLoadingAssets(label)
    try {
        const response = await fetch(fetchStr)
        if (!response.ok) return response
        return await response.arrayBuffer()
    } catch (error) {
        warn(true, error)
        return failed(500)
    } finally {
        API.Misc.stopCurrentlyLoadingAssets(label)
    }
}

/**
 * The parts of the upstream API the renderer uses.
 * @category API
 */
export const API = {
    "Misc": {
        "startCurrentlyLoadingAssets": function (label: string) {
            currentlyLoadingAssets.push(label)
            _updateCurrentlyLoadingAssets("start", label)
        },
        "stopCurrentlyLoadingAssets": function(label: string) {
            const labelIndex = currentlyLoadingAssets.indexOf(label)
            if (labelIndex > -1) {
                currentlyLoadingAssets.splice(labelIndex, 1)
                _updateCurrentlyLoadingAssets("finish", label)
            }
        },
        "idFromStr": function(str: string) {
            const numStrs = str.match(/\d+(\.\d+)?/g) || []
            return numStrs.length > 0 ? Number(numStrs[numStrs.length - 1]) : NaN
        },
        /** A stable key for an asset string. Upstream returned a fetch URL here. */
        "parseAssetString": function(str: string) {
            const r = resolveAsset(str)
            if (!r) return str
            return "url" in r ? r.url : `rbxassetid://${r.id}`
        },
        "assetURLToCDNURL": async function(url: string | number | bigint, headers?: HeadersInit): Promise<string | Response> {
            return assetUrl(String(url), headers) ?? failed()
        },
        "getCurrentlyLoading": function(): boolean {
            return currentlyLoadingAssets.length > 0
        },
    },
    "Events": {
        "OnLoadingAssets": new Event()
    },
    "Generic": {
        LoadImage: async function(url: string): Promise<HTMLImageElement | undefined> {
            const cacheURL = API.Misc.parseAssetString(url)
            const cached = CACHE.Image.get(cacheURL)
            if (cached) return cached

            const promise = new Promise<HTMLImageElement | undefined>((resolve) => {
                const src = assetUrl(url)
                if (!src) {
                    resolve(undefined)
                    return
                }
                const image = new Image()
                image.onload = () => resolve(image)
                image.onerror = () => resolve(undefined)
                image.crossOrigin = "anonymous"
                image.src = src
            })
            CACHE.Image.set(cacheURL, promise)
            const image = await promise
            CACHE.Image.set(cacheURL, image)
            return image
        },
    },
    "Asset": {
        GetAssetBuffer: async function(url: string, headers?: HeadersInit, representations?: string): Promise<Response | ArrayBuffer> {
            let cacheStr = API.Misc.parseAssetString(url)
            if (headers) cacheStr += JSON.stringify(headers)
            if (representations) cacheStr += representations

            const cachedBuffer = CACHE.AssetBuffer.get(cacheStr)
            if (cachedBuffer) return cachedBuffer

            const promise = getAssetBufferInternal(url, headers, representations)
            if (FLAGS.ENABLE_API_CACHE) {
                CACHE.AssetBuffer.set(cacheStr, promise)
            }
            return promise
        },
        /**
         * Remember to call .Destroy() on the Instance returned by RBX.generateTree() to avoid memory leaks
         */
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        GetRBX: async function(url: string, headers?: HeadersInit, contentRepresentationPriorityList?: any): Promise<Response | RBX> {
            const representations = contentRepresentationPriorityList ? btoa(JSON.stringify(contentRepresentationPriorityList)) : undefined

            let cacheStr = url
            if (headers) cacheStr += JSON.stringify(headers)
            if (representations) cacheStr += representations

            const cachedRBX = CACHE.RBX.get(cacheStr)
            if (cachedRBX) return cachedRBX.clone()

            const response = await API.Asset.GetAssetBuffer(url, headers, representations)
            if (!(response instanceof ArrayBuffer)) return response
            const rbx = new RBX()
            try {
                rbx.fromBuffer(response)
            } catch {
                return failed(422)
            }
            if (FLAGS.ENABLE_API_CACHE && FLAGS.ENABLE_API_RBX_CACHE) {
                CACHE.RBX.set(cacheStr, rbx.clone())
            }
            return rbx
        },
        GetMesh: async function(url: string, headers?: HeadersInit, readOnly: boolean = false): Promise<FileMesh | Response> {
            let cacheStr = url
            if (headers) cacheStr += JSON.stringify(headers)

            const cachedMesh = CACHE.Mesh.get(cacheStr)
            if (cachedMesh) return readOnly ? cachedMesh : cachedMesh.clone()

            const response = await API.Asset.GetAssetBuffer(url, headers)
            if (!(response instanceof ArrayBuffer)) {
                warn(true, "Failed to get mesh, response was not an ArrayBuffer")
                return response
            }
            const mesh = new FileMesh()
            try {
                await mesh.fromBuffer(response)
            } catch { // draco decode or something else might fail
                warn(true, "Failed to get mesh, mesh.fromBuffer failed, is DracoDecoderModule available?")
                return failed(422)
            }
            if (FLAGS.ENABLE_API_CACHE && FLAGS.ENABLE_API_MESH_CACHE) {
                CACHE.Mesh.set(cacheStr, readOnly ? mesh : mesh.clone())
            }
            return mesh
        },
        GetAssetTypeId: async function(_assetId: string | number): Promise<number | Response> {
            return failed(501)
        },
        IsLayered: async function(id: number): Promise<boolean | Response> {
            const cached = CACHE.IsLayered.get(id)
            if (cached !== undefined) return cached

            const result = await API.Asset.GetRBX(`rbxassetid://${id}`, undefined)
            if (!(result instanceof RBX)) {
                warn(true, "Failed to get accessory")
                return result
            }
            const dataModel = result.generateTree()
            const hasWrapLayer = dataModel.GetDescendants().some((child) => child.className === "WrapLayer")
            CACHE.IsLayered.set(id, hasWrapLayer)
            dataModel.Destroy()
            return hasWrapLayer
        },
    },
    // Metadata lookups used only by Outfit helpers the app doesn't call (addAssetId, addBundleId,
    // fromLook). The app passes complete outfits instead.
    "Catalog": {
        GetItemDetails: async function(_auth: Authentication, _items: {itemType: "Asset" | "Bundle", id: number}[]): Promise<Response | ItemDetails_Result> {
            return failed(501)
        },
        GetBundleDetails: async function(_bundleId: number | string): Promise<Response | BundleDetails_Result> {
            return failed(501)
        },
    },
    "Economy": {
        GetAssetDetails: async function(_assetId: number): Promise<Response> {
            return failed(501)
        },
    },
    "Avatar": {
        GetOutfitDetails: async function(_outfitId: number | string, _userId: number): Promise<Response | Outfit> {
            return failed(501)
        },
    },
}
