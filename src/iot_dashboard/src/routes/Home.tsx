import SensorDisplayManager from "../components/SensorDisplayManager"

const Home = () => {
  return (
    <div>
      <SensorDisplayManager realTimeRefetchRate={4000} sensorRefetchRate={40000} />
    </div>
  )
}

export default Home
