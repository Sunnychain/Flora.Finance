import {
  BrowserRouter as Router,
  Route,
  Redirect,
  Switch
} from 'react-router-dom';

import About from './pages/About/About';
import Contact from './pages/Contact/Contact';
import Home from './pages/Home/Home';
import CarbonCredits from './pages/CarbonCredits/CarbonCredits';
import NFT from './pages/Nft/NFT';
import Navbar from './Components/Navbar/Navbar';
import Projects from './pages/projects/Projects';
import Dev from './Components/dev/developerPage';
const App = () => {
  return (
   <Router>
    <Navbar/>
    <main>
      <Switch>
        <Route path="/" exact>
          <Home/>
        </Route>
        <Route path="/about" exact>
          <About/>
        </Route>
        <Route path="/projects" exact>
          <Projects/>
        </Route>
        <Route path="/carbon-credits" exact>
          <CarbonCredits/>
        </Route>
        <Route path="/nft" exact>
          <NFT/>
        </Route>
        <Route path="/contact" exact>
          <Contact/>
        </Route>
        <Route path="/in-dev" exact>
          <Dev/>
        </Route>
        <Redirect to="/" />
      </Switch>
    </main>
   </Router>
  );
}

export default App;