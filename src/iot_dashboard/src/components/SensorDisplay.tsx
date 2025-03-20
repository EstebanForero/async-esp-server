import { SensorValues } from '../backend/backend'

interface SensorDisplayProps {
  title: string;
  realTimeValue: number | boolean | undefined;
  historyValues: SensorValues[] | undefined;
  unit: string;
  formatValue: (value: number | boolean) => string;
}

export function SensorDisplay(props: SensorDisplayProps) {
  return (
    <section class="sensor-section">
      <div class="real-time">
        <h2>{props.title}</h2>
        <p>
          <strong>Real-time:</strong>{" "}
          {props.realTimeValue !== undefined
            ? props.formatValue(props.realTimeValue)
            : "Loading..."}
          {props.unit}
        </p>
      </div>
      <div class="history">
        <h3>History</h3>
        <ul>
          {props.historyValues?.map((val, _) => (
            <li>
              {props.formatValue(
                props.title === "Temperature"
                  ? val.temp
                  : props.title === "Gas"
                    ? val.gas
                    : val.flame
              )}
              {props.unit}
            </li>
          )) ?? "Loading..."}
        </ul>
      </div>
    </section>
  );
}

export default SensorDisplay
