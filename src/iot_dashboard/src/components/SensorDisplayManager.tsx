import { createEffect, createResource, onCleanup } from "solid-js";
import SensorDisplay from "./SensorDisplay";
import { fetchRealTimeSensorValues, fetchSensorValues, fetchSensorValuesHistory, SensorValues, SensorValuesInfo, ValueHistoryArray } from "../backend/backend";

interface Props {
  realTimeRefetchRate: number;
  sensorRefetchRate: number;
}

const SensorDisplayManager = (props: Props) => {
  const [historyData, { mutate: mutateHistory }] = createResource<ValueHistoryArray>(fetchSensorValuesHistory);
  const [currentSensorData, { refetch: refetchCurrentSensorData }] = createResource<SensorValuesInfo>(fetchSensorValues);
  const [realTimeData, { refetch: refetchRealTimeData, }] = createResource<SensorValues>(fetchRealTimeSensorValues);

  const sensorInterval = setInterval(() => {
    console.log("Fetching current sensor data...");
    refetchCurrentSensorData();
  }, props.sensorRefetchRate, props.sensorRefetchRate / 2);

  const realTimeInterval = setInterval(() => {
    console.log("Fetching real-time sensor data...");
    console.log("Real-time data:", JSON.stringify(realTimeData()));
    refetchRealTimeData();
  }, props.realTimeRefetchRate);

  onCleanup(() => {
    clearInterval(sensorInterval);
    clearInterval(realTimeInterval);
  });

  createEffect(() => {
    let currentData = currentSensorData();
    if (currentData?.has_changed) {
      mutateHistory((prev) => ({
        values: prev ? [...prev.values, currentData.sensor_values] : [currentData.sensor_values],
      }));
    }
  });

  return (
    <div class="home-container">
      <h1>Sensor Data Dashboard</h1>
      <SensorDisplay title="Temperature" realTimeValue={realTimeData()?.temp} historyValues={historyData()?.values} unit=" °C" formatValue={(v) => (typeof v === "number" ? v.toFixed(2) : "N/A")} />
      <SensorDisplay title="Gas" realTimeValue={realTimeData()?.gas} historyValues={historyData()?.values} unit=" ppm" formatValue={(v) => (typeof v === "number" ? v.toString() : "N/A")} />
      <SensorDisplay title="Flame" realTimeValue={realTimeData()?.flame} historyValues={historyData()?.values} unit="" formatValue={(v) => (v ? "Detected" : "Not Detected")} />
    </div>
  );
};


export default SensorDisplayManager
