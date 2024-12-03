import {BrowserRouter, Routes, Route} from 'react-router-dom'
import './styles/App.css'

import WelcomePage from './pages/Welcome'
import SignInPage from './pages/Sign'

function App() {
  return (
    <div>
      <BrowserRouter>
        <Routes>
          <Route path="/" element={<WelcomePage />} />
          <Route path="/signin" element={<SignInPage />} />
        </Routes>
      </BrowserRouter>
    </div>
  )
}

export default App
