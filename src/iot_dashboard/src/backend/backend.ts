export type RiskLevels = "Low" | "Moderate" | "High" | "Normal";

export interface EspConfig {
  temp_threshold: number;
  gas_threshold: number;
  alarms_enabled: boolean;
  data_point_interval: number;
}

export interface SensorValues {
  temp: number;
  gas: number;
  flame: boolean;
}

export interface SensorValuesInfo {
  sensor_values: SensorValues;
  has_changed: boolean;
}

export interface ValueHistoryArray {
  values: SensorValues[];
}

//const BASE_URL = "http://172.20.10.3";
const BASE_URL = `${window.location.protocol}//${window.location.host}`;

export const fetchConfig = async (): Promise<EspConfig> => fetchJson<EspConfig>(`${BASE_URL}/config`);

export const updateConfig = async (config: EspConfig): Promise<void> => postJson(`${BASE_URL}/config`, config);

export const fetchSensorValues = async (): Promise<SensorValuesInfo> => {
  const text = await fetchText(`${BASE_URL}/values`);
  const [valuesPart, hasChangedPart] = text.split(" ");
  return { sensor_values: parseSensorValues(valuesPart), has_changed: hasChangedPart === "1" };
};

export const fetchRealTimeSensorValues = async (): Promise<SensorValues> => {
  const text = await fetchText(`${BASE_URL}/values/now`);
  const parsedValues = parseSensorValues(text);
  //console.log('The parsed values are: ', JSON.stringify(parsedValues))
  return parsedValues
};

export const fetchSensorValuesHistory = async (): Promise<ValueHistoryArray> => {
  const text = await fetchText(`${BASE_URL}/values/history`);
  const values = text.split("|").map(parseSensorValues);
  //console.log('The values history is: ', values)
  return { values };
};

async function fetchText(url: string): Promise<string> {
  const response = await fetch(url);
  if (!response.ok) throw new Error(`Failed to fetch data from ${url}: ${response.statusText}`);
  return response.text();
}

async function fetchJson<T>(url: string): Promise<T> {
  const response = await fetch(url);
  if (!response.ok) throw new Error(`Failed to fetch JSON from ${url}: ${response.statusText}`);
  return response.json();
}

async function postJson(url: string, data: any): Promise<void> {
  const response = await fetch(url, { method: "POST", body: JSON.stringify(data) });
  if (!response.ok) throw new Error(`Failed to post data to ${url}: ${response.statusText}`);
}

function parseSensorValues(valuesStr: string): SensorValues {
  const [temp, gas, flame] = valuesStr.split(",");
  return { temp: parseFloat(temp), gas: parseInt(gas, 10), flame: flame === "1" };
}
