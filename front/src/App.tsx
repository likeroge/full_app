import { useNavigate } from 'react-router'
import './App.css'

function App() {

  const navigate = useNavigate()

  const onButtonHandler = () => {
    navigate('/hello')
  }
  return (
    <div>
      <h1>Hello world</h1>
      <button onClick={onButtonHandler}>Go to next page</button>
    </div>
  )
}

export default App
