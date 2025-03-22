import { RiskLevels } from "../backend/backend";
import { getConfigStore } from "../backend/configStore";
import { createEffect, createSignal, Show } from "solid-js";
import { SensorValues } from "../backend/backend";

type Props = {
  sensorValues?: SensorValues;
};

const RiskNotifier = (props: Props) => {
  const [risk, setRisk] = createSignal<RiskLevels>("Normal");

  createEffect(() => {
    const config = getConfigStore();
    //console.log("RiskNotifier rendering, sensorValues:", JSON.stringify(props.sensorValues));
    //console.log("The config is: ", JSON.stringify(config))

    if (!config || !props.sensorValues) {
      //console.log("Effect skipped: config or sensorValues missing");
      return;
    }

    console.log("Effect running, sensorValues:", JSON.stringify(props.sensorValues), "config:", JSON.stringify(config));

    if (props.sensorValues.flame) {
      setRisk("High");
      //console.log("setting high risk");
      return;
    }

    if (props.sensorValues.gas > config.gas_threshold && props.sensorValues.temp > config.temp_threshold) {
      setRisk("High");
      //console.log("setting high risk");
      return;
    }

    if (props.sensorValues.gas > config.gas_threshold || props.sensorValues.temp > config.temp_threshold) {
      setRisk("Moderate");
      //console.log("setting moderate risk");
      return;
    }

    //console.log("setting low risk");
    setRisk("Low");
  })


  return (
    <div class="notification-container">
      <Show when={risk() !== "Normal"}>
        <div class={`notification ${risk().toLowerCase()}-risk`}>
          {risk()} risk
        </div>
      </Show>
    </div>
  );
};

export default RiskNotifier;
