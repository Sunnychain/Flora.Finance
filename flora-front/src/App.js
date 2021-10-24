import React from 'react';
import { Route, BrowserRouter as Router, Switch } from 'react-router-dom';
import Main from './middlewares/Main';
import Home from './pages/Home/Home';
import NotFound from './pages/NotFound/NotFound';
import Market from './pages/market/Market';
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
          <Route path="/404" exact>
            <NotFound />
          </Route>
        </Switch>
      </Router>
    </SubstrateContextProvider>
  );
}
