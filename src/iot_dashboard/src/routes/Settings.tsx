import { createEffect, createResource, Show } from "solid-js";
import { EspConfig, fetchConfig, updateConfig } from "../backend/backend";
import { setConfigStore } from "../backend/configStore";

const Settings = () => {
  const [configResource, { mutate, refetch }] = createResource(fetchConfig);

  const handleInputChange = (field: keyof EspConfig, value: number | boolean) => {
    if (configResource()) {
      let prevEspConfg = configResource()
      if (prevEspConfg) {
        let espConfig: EspConfig | undefined = { ...prevEspConfg, [field]: value };
        mutate(espConfig)
      }
    }
  };

  const handleSave = async () => {
    const espConfig = configResource()
    if (espConfig) {
      await updateConfig(espConfig);
      setConfigStore(espConfig)
      refetch()
    }
  };

  createEffect(() => {
    const espConfig = configResource()
    if (espConfig) {
      setConfigStore(espConfig)
    }
  })

  return (
    <div class="settings-container">
      <h1>Settings</h1>
      <Show when={!configResource.loading} fallback={<p>Loading...</p>}>
        <Show when={!configResource.error} fallback={<p>Error loading configuration.</p>}>
          <form class="settings-form">
            <label>
              Temperature Threshold:
              <input
                type="number"
                value={configResource()?.temp_threshold || 0}
                onInput={(e) => handleInputChange("temp_threshold", parseFloat(e.target.value))}
              />
            </label>
            <label>
              Gas Threshold:
              <input
                type="number"
                value={configResource()?.gas_threshold || 0}
                onInput={(e) => handleInputChange("gas_threshold", parseFloat(e.target.value))}
              />
            </label>
            <label>
              Alarms Enabled:
              <input
                type="checkbox"
                checked={configResource()?.alarms_enabled || false}
                onChange={(e) => handleInputChange("alarms_enabled", e.target.checked)}
              />
            </label>
            <label>
              Data Point Interval:
              <input
                type="number"
                value={configResource()?.data_point_interval || 0}
                onInput={(e) => handleInputChange("data_point_interval", parseInt(e.target.value, 10))}
              />
            </label>
            <button type="button" onClick={handleSave} class="saveButton">
              Save
            </button>
          </form>
        </Show>
      </Show>
    </div>
  );
};

export default Settings;
