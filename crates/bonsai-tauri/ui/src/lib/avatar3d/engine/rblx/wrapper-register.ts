import { InstanceWrapper } from "./instance/InstanceWrapper";

/* eslint-disable @typescript-eslint/no-explicit-any */
const modules = import.meta.glob('./instance/*', { eager: true });

export function RegisterWrappers() {
    for (const module of Object.values(modules)) {
        for (const exprt of Object.values(module as any)) {
            //go up hierarchy until we directly inherit InstanceWrapper
            let prototype = Object.getPrototypeOf(exprt as any)
            while (prototype) {
                if (prototype === InstanceWrapper) {
                    (exprt as typeof InstanceWrapper).register()
                    break
                }

                prototype = Object.getPrototypeOf(prototype)
            }
        }
    }
}