import { EspConfig } from "./backend";

let configStore: EspConfig | undefined = undefined;

export function setConfigStore(config: EspConfig) {
  console.log('config store is being setted')
  configStore = config
}

export function getConfigStore(): EspConfig | undefined {
  return configStore
}
