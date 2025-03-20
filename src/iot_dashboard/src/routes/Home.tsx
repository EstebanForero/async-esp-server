import SensorDisplayManager from "../components/SensorDisplayManager"

const Home = () => {
  return (
    <div>
      <SensorDisplayManager realTimeRefetchRate={8000} sensorRefetchRate={8000} />
    </div>
  )
}

export default Home
