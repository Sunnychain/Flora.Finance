
---

```js
import React from 'react';
import { Route, BrowserRouter as Router, Switch } from 'react-router-dom';
import Main from './middlewares/Main';
import Home from './pages/Home/Home';
import NotFound from './pages/NotFound/NotFound';
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
          <Route path="/404" exact>
            <NotFound />
          </Route>
        </Switch>
      </Router>
    </SubstrateContextProvider>
  );
}

```


src/config/development.json

TerminalConnectedNodeSocket

```js

{
  "PROVIDER_SOCKET": "ws://127.0.0.1:9944"
}

```

API - SubstrateContext.js
```js
//React.createContext https://reactjs.org/docs/context.html#reactcreatecontext
//const MyContext = React.createContext(defaultValue);
const SubstrateContext = React.createContext();

const SubstrateContextProvider = (props) => {
  //...
    const neededPropNames = ['socket', 'types'];
    const initState = { ...INIT_STATE };                 -------------------->
  //...

const useSubstrate = () => ({ ...useContext(SubstrateContext) });

export { SubstrateContextProvider, useSubstrate };
```







SubstrateContext e useSubstrate

"Main.js":
O react interage com o nó do Substrate utilizando-se do "Context" do React (Context provides a way to pass data through the component tree without having to pass props down manually at every level.) dentro do Hook (Hooks are a new addition in React 16.8. They let you use state and other React features without writing a class.)

```js
//...
import { SubstrateContextProvider, useSubstrate } from './substrate-lib';
import { DeveloperConsole } from './substrate-lib/components';


{
  const [accountAddress, setAccountAddress] = useState(null);               ------>
  const { apiState, keyring, keyringState, apiError } = useSubstrate();     ------>

  export default function Main () {
  const [accountAddress, setAccountAddress] = useState(null);
  const { apiState, keyring, keyringState, apiError } = useSubstrate();
  const accountPair = 
    accountAddress &&
    keyringState === 'READY' &&
    keyring.getPair(accountAddress);

  if (apiState === 'ERROR') return message(apiError);
  else if (apiState !== 'READY') return loader('Connecting to Substrate');

  if (keyringState !== 'READY') {
    return loader('Loading accounts (please review any extension\'s authorization)');
  }


  const contextRef = createRef();
  
  return (
  ...
  );
}
}
```

App.js

```js

//...
import { SubstrateContextProvider } from './substrate-lib';
//...


export default function App () {
  return (
    <SubstrateContextProvider>               ------>
      <Router>
        <Switch>
        <Route path="/" exact>
            <Home />
          </Route>
          <Route path="/create" exact>  
            <Main />
          </Route>
          <Route path="/404" exact>
            <NotFound />
          </Route>
        </Switch>
      </Router>
    </SubstrateContextProvider>               ------>
  );
}
```



https://dmitripavlutin.com/react-usereducer/
HOOK
import React, { useReducer, useContext } from 'react';

```js
const INIT_STATE = {
  socket: connectedSocket,
  jsonrpc: { ...jsonrpc, ...config.RPC },
  types: config.types,
  keyring: null,
  keyringState: null,
  api: null,
  apiError: null,
  apiState: null
};

const reducer = (state, action) => {
  switch (action.type) {
    case 'CONNECT_INIT':
      return { ...state, apiState: 'CONNECT_INIT' };

    case 'CONNECT':
      return { ...state, api: action.payload, apiState: 'CONNECTING' };

    case 'CONNECT_SUCCESS':
      return { ...state, apiState: 'READY' };

    case 'CONNECT_ERROR':
      return { ...state, apiState: 'ERROR', apiError: action.payload };

    case 'LOAD_KEYRING':
      return { ...state, keyringState: 'LOADING' };

    case 'SET_KEYRING':
      return { ...state, keyring: action.payload, keyringState: 'READY' };

    case 'KEYRING_ERROR':
      return { ...state, keyring: null, keyringState: 'ERROR' };

    default:
      throw new Error(`Unknown type: ${action.type}`);
  }
};
```

```js
API - DeveloperConsole.js

import { useSubstrate } from '../';

export default function DeveloperConsole (props) {
  const { api, apiState, keyring, keyringState } = useSubstrate();
  if (apiState === 'READY') { window.api = api; }
  if (keyringState === 'READY') { window.keyring = keyring; }
  window.util = require('@polkadot/util');
  window.utilCrypto = require('@polkadot/util-crypto');

  return null;
}
```


socket - Corresponde à extremidade remota atualmente conectada
types - estrutura personalizada dentro do nó do substrate
keyring - armazena contas de usuário (chaves públicas) e também abre interfaces para assinar dados e transações
keyringState - Status da conta do usuário, um entre: [null, 'READY', 'ERROR']
api - POLKADOT-JS API
apiState - Status da conexão da API do Polkadot-JS com a extremidade remota (socket), um entre: [null, 'CONNECTING', 'READY', 'ERROR']

Logo, verifica-se apiState e keyringState, quando seus valores forem 'READY', podemos começar a ler os dados da blockchain/nó.

Ler e assinar os dados no nó do substrate/blockchain (Queries)
Portanto, falaremos do nosso arquivo: flora-front/src/substrate-lib/components/TxButton.js

Nesse momento lidaremos com a leitura de dados do nó, envio de transações e monitoramento de eventos.

** Esse arquivo front-end corresponde à um PALLET do nó do Substrate no backend**
Nele temos:
Um item de acesso - something
Uma interface de leitura - something
Uma interface de transação externa (extrinsic) do_something

Vejamos a interface

1 -
2 -


```js 
//... flora-front/src/components/AccountSelector.jsAccountSelector.js
function Main (props) {
  const { keyring } = useSubstrate();
//...

export default function AccountSelector (props) {
  const { api, keyring } = useSubstrate();
  return keyring.getPairs && api.query ? <Main {...props} /> : null;
}


//...flora-front/src/components/BlockNumber.js
function Main (props) {
  const { api } = useSubstrate();
//...

export default function BlockNumber (props) {
  const { api } = useSubstrate();
  return api.derive &&
    api.derive.chain &&
    api.derive.chain.bestNumber &&
    api.derive.chain.bestNumberFinalized
    ? <Main {...props} />
    : null;
}

//...flora-front/src/components/Events.js
function Main (props) {
  const { api } = useSubstrate();
//...

export default function Events (props) {
  const { api } = useSubstrate();
  return api.query && api.query.system && api.query.system.events
    ? <Main {...props} />
    : null;
}

//...flora-front/src/components/Interactor.js
function Main (props) {
  const { api, jsonrpc } = useSubstrate();
//...

export default function Interactor (props) {
  const { api } = useSubstrate();
  return api.tx ? <Main {...props} /> : null;
}

//...flora-front/src/components/NodeInfo.js
function Main (props) {
  const { api, socket } = useSubstrate();
//...

export default function NodeInfo (props) {
  const { api } = useSubstrate();
  return api.rpc &&
    api.rpc.system &&
    api.rpc.system.chain &&
    api.rpc.system.name &&
    api.rpc.system.version
    ? <Main {...props} />
    : null;
}

//...flora-front/src/middlewares/Main.js
export default function Main () {
  const { apiState, keyring, keyringState, apiError } = useSubstrate();
//...

//...flora-front/src/substrate-lib/components/DeveloperConsole.js
export default function DeveloperConsole (props) {
  const { api, apiState, keyring, keyringState } = useSubstrate();
//...

//...flora-front/src/substrate-lib/components/TxButton.js
function TxButton ({
  //...
}) {
  // Hooks
  const { api } = useSubstrate();
//...

}

```

