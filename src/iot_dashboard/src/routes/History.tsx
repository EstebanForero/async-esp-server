import { createResource, ErrorBoundary, onCleanup, resetErrorBoundaries } from "solid-js";
import { fetchSensorValuesHistory, ValueHistoryArray } from "../backend/backend";


interface Props {
  sensorRefetchRate: number;
}

const History = (props: Props) => {
  const [historyData, { refetch }] = createResource<ValueHistoryArray>(fetchSensorValuesHistory);

  const historyInterval = setInterval(() => {
    if (!historyData.loading) {
      console.log("Fetching sensor history data...");
      refetch();

    }
    resetErrorBoundaries()
  }, props.sensorRefetchRate);

  onCleanup(() => {
    clearInterval(historyInterval);
  });

  return (
    <ErrorBoundary fallback={<div>Error...</div>}>
      <div class="history-container">
        <h2>Historical Values</h2>
        <section class="sensor-history">
          <h3>Temperature History</h3>
          <ul>
            {historyData()?.values?.reverse().map((val) => (
              <li>{val.temp.toFixed(2)} °C</li>
            )) ?? "Loading..."}
          </ul>
        </section>
        <section class="sensor-history">
          <h3>Gas History</h3>
          <ul>
            {historyData()?.values?.reverse().map((val) => (
              <li>{val.gas.toString()} ppm</li>
            )) ?? "Loading..."}
          </ul>
        </section>
        <section class="sensor-history">
          <h3>Flame History</h3>
          <ul>
            {historyData()?.values?.reverse().map((val) => (
              <li>{val.flame ? "Detected" : "Not Detected"}</li>
            )) ?? "Loading..."}
          </ul>
        </section>
      </div>
    </ErrorBoundary>
  );
};

export default History;
