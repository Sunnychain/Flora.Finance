import React from 'react';
import { Route, BrowserRouter as Router, Switch } from 'react-router-dom';
import Main from './middlewares/Main';
import Home from './pages/Home/Home';
import About from './pages/About/About';
import Market from './pages/market/Market';
import Contact from './pages/Contact/Contact';
import { SubstrateContextProvider } from './substrate-lib';
export default function App () {
  return (
    <SubstrateContextProvider>
      <Router>
        <Switch>
        <Route path="/" exact>
            <Home />
          </Route>
          <Route path="/create" exact>
            <Main />
          </Route>
          <Route path="/market" exact>
            <Market />
          </Route>
          <Route path="/about" exact>
            <About />
          </Route>
          <Route path="/contact" exact>
            <Contact />
          </Route>
            <Route path="/contact" exact>
            <Contact />
          </Route>
        </Switch>
      </Router>
    </SubstrateContextProvider>
  );
}
