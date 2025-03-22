import { createSignal } from "solid-js/types/server/reactive.js"
import { RiskLevels } from "../backend/backend"
import { getConfigStore } from "../backend/configStore"
import { onMount, Show } from "solid-js"
import { SensorValues } from "../backend/backend"

type Props = {
  sensorValues?: SensorValues
}

const RiskNotifier = ({ sensorValues }: Props) => {
  let config = getConfigStore()
  const [risk, setRisk] = createSignal<RiskLevels>('Normal');

  onMount(() => {
    config = getConfigStore()

    if (!config || !sensorValues) {
      return;
    }

    if (sensorValues.flame) {
      setRisk('High')
      return
    }

    if (sensorValues.gas > config.gas_threshold && sensorValues.temp > config.temp_threshold) {
      setRisk('High')
      return
    }

    if (sensorValues.gas > config.gas_threshold || sensorValues.temp > config.temp_threshold) {
      setRisk('Moderate')
      return
    }

    setRisk('Low')
  })



  return (
    <div>
      <Show when={risk() != "Normal"}>
        <div>
          <Show when={risk() == "Low"}>
            Low risk
          </Show>
          <Show when={risk() == "Moderate"}>
            Moderate risk
          </Show>
          <Show when={risk() == "High"}>
            High risk
          </Show>
        </div>
      </Show>
    </div>
  )
}

export default RiskNotifier
