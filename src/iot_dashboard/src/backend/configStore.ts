import { EspConfig } from "./backend";

let configStore: EspConfig | undefined = undefined;

export function setConfigStore(config: EspConfig) {
  configStore = config
}

export function getConfigStore(): EspConfig | undefined {
  return configStore
}
