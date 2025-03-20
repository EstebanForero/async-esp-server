import { createEffect, createResource } from "solid-js";
import SensorDisplay from "./SensorDisplay";
import { fetchRealTimeSensorValues, fetchSensorValues, fetchSensorValuesHistory, SensorValues, SensorValuesInfo, ValueHistoryArray } from "../backend/backend";

interface Props {
  realTimeRefetchRate: number;
  sensorRefetchRate: number;
}

const SensorDisplayManager = (props: Props) => {
  const [historyData, { mutate: mutateHistory }] = createResource<ValueHistoryArray>(fetchSensorValuesHistory);

  const [currentSensorData, { refetch: refetchCurrentSensorData }] = createResource<SensorValuesInfo>(fetchSensorValues);

  const timeCurrentSensor = () => {
    console.log('timeout current values called')
    refetchCurrentSensorData()
    setTimeout(() => timeCurrentSensor(), props.sensorRefetchRate)
  }

  setTimeout(() => {
    timeCurrentSensor()
  }, props.sensorRefetchRate + (props.realTimeRefetchRate / 2))

  createEffect(() => {
    let currentData = currentSensorData()
    if (currentData?.has_changed) {
      mutateHistory(prev => {
        let x: ValueHistoryArray = { values: prev ? [...prev.values, currentData.sensor_values] : [currentData.sensor_values] }
        return x
      })
    }
  })

  const [realTimeData, { refetch: refetchRealTimeData }] = createResource<SensorValues>(fetchRealTimeSensorValues);

  const timeOutRealTime = () => {
    console.log('timeout real time called')
    refetchRealTimeData()
    setTimeout(() => timeOutRealTime(), props.realTimeRefetchRate)
  }

  setTimeout(() => {
    timeOutRealTime()
  }, props.realTimeRefetchRate)

  const formatTemp = (value: number | boolean) => (typeof value === "number" ? value.toFixed(2) : "N/A");
  const formatGas = (value: number | boolean) => (typeof value === "number" ? value.toString() : "N/A");
  const formatFlame = (value: number | boolean) => (value ? "Detected" : "Not Detected");

  return (
    <div class="home-container">
      <h1>Sensor Data Dashboard</h1>

      <SensorDisplay
        title="Temperature"
        realTimeValue={realTimeData()?.temp}
        historyValues={historyData()?.values}
        unit=" °C"
        formatValue={formatTemp}
      />
      <SensorDisplay
        title="Gas"
        realTimeValue={realTimeData()?.gas}
        historyValues={historyData()?.values}
        unit=" ppm"
        formatValue={formatGas}
      />
      <SensorDisplay
        title="Flame"
        realTimeValue={realTimeData()?.flame}
        historyValues={historyData()?.values}
        unit=""
        formatValue={formatFlame}
      />
    </div>
  );
}

export default SensorDisplayManager
