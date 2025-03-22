import { createResource, ErrorBoundary, onCleanup, resetErrorBoundaries } from "solid-js";
import { fetchRealTimeSensorValues, SensorValues } from "../backend/backend";
import RiskNotifier from "../components/RiskNotifier";

interface Props {
  realTimeRefetchRate: number;
}

const Values = (props: Props) => {
  const [realTimeData, { refetch }] = createResource<SensorValues>(fetchRealTimeSensorValues);

  const realTimeInterval = setInterval(() => {
    if (!realTimeData.loading) {
      //console.log("Fetching real-time sensor data...");
      refetch();
      resetErrorBoundaries()
    }
  }, props.realTimeRefetchRate);

  onCleanup(() => {
    clearInterval(realTimeInterval);
  });

  return (
    <ErrorBoundary fallback={<div>Error...</div>} >
      <RiskNotifier sensorValues={realTimeData()} />
      <div class="values-container">
        <h2>Real-Time Values</h2>
        <section class="sensor-section">
          <div class="real-time">
            <h3>Temperature</h3>
            <p>
              {realTimeData()?.temp !== undefined
                ? `${realTimeData()?.temp.toFixed(2)} °C`
                : "Loading..."}
            </p>
          </div>
        </section>
        <section class="sensor-section">
          <div class="real-time">
            <h3>Gas</h3>
            <p>
              {realTimeData()?.gas !== undefined
                ? `${realTimeData()?.gas.toString()} ppm`
                : "Loading..."}
            </p>
          </div>
        </section>
        <section class="sensor-section">
          <div class="real-time">
            <h3>Flame</h3>
            <p>
              {realTimeData()?.flame !== undefined
                ? (realTimeData()?.flame ? "Detected" : "Not Detected")
                : "Loading..."}
            </p>
          </div>
        </section>
      </div>
    </ErrorBoundary>
  );
};

export default Values;
