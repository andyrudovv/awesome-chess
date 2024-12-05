import {BrowserRouter, Routes, Route} from 'react-router-dom'
import './styles/App.css'

import WelcomePage from './pages/Welcome'
import SignInPage from './pages/Sign'
import HomePage from './pages/Home'
import MoleculesBackground from './MoleculesBackground'


function App() {
  return (
    <div>
      <BrowserRouter>
        <div className='app-container'>
          <MoleculesBackground id='particles' />
          <Routes>
            <Route path="/" element={<WelcomePage />} />
            <Route path="/signin" element={<SignInPage />} />
            <Route path="/home" element={<HomePage />} />
          </Routes>
        </div>
      </BrowserRouter>
    </div>
  )
}

export default App
