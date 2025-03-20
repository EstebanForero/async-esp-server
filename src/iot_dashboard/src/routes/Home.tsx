import SensorDisplayManager from "../components/SensorDisplayManager"

const Home = () => {
  return (
    <div>
      <SensorDisplayManager realTimeRefetchRate={1000} sensorRefetchRate={1000} />
    </div>
  )
}

export default Home
