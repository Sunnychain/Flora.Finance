
----------------------------------------------------------------------------------------------------


Parte 1

Processo de criação do Flora.finance front-end e interação com o node do Substrate utilizando-se da API Polkadot-JS,
pois é pelo mesmo que os usuários interagirão com a blockchain.

-----------------
O conteúdo a seguir é adequado para qualquer estrutura de front-end, mas utilizaremos um middleware (é todo o tipo de função que está entre um pedido HTTP e a resposta final que o servidor/node/backend envia de volta para o cliente.), pois a API também permite isso.

"A função middleware tem 3 parametros, pedido, resposta e callback. Podes ter n middleware a processar um pedido HTTP, encadeados. Quando um middleware acaba de processar coloca-se no final do código next();, invocando assim a callback e o código continua a correr para o proximo middleware ou resposta final.

O middleware é portanto uma funcionalidade, funções que executam processos intermédios. Os exemplos mais comuns são interagir com a BD, ir buscar ficheiros estáticos, tratar de erros ou redirecionamentos."
-----------------

Nos beseamos no modelo substrate-front-end-template, que encapsula a API Polkadot-JS em componentes React.

A porta do socket (wo-way communication) é localhost:9944:

----------------- flora-front/src/config/development.json
### Connect with Polkadot-JS Apps Front-end

Once the node template is running locally, you can connect it with **Polkadot-JS Apps** front-end
to interact with your chain. [Click
here](https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944) connecting the Apps to your
local node template.
-----------------

Se for um projeto novo, faz-se necessário incluir a biblioteca API JS ao projeto:

```bash
yarn add @polkadot/api
```

Para nosso caso, utilizamos o yarn(ou npm) install para instalar as dependências e a API está inclusa.


"I swear, it feels like it’s been forever since we’ve been using promises in JavaScript, but they actually only became part of the JavaScript language with the ES2015 standard."

"A promise is just that: a promise to either return a result at some point in the future or a promise to reject with an error at some point in the future. When a promise knows the return result, the promise resolves with the return result. When a promise fails because of an error, the promise rejects with the reason why the promise failed."

Kerri Shotts, https://medium.com/@photokandy/es2015-promises-es2017-async-await-f2a060bb38ce

Seguindo essa premissa para começar a trabalhar com o node e o front, utiliz-se o comando "import" (para referenciar bibliotecas externas) tanto do "ApiPromise" quanto "WsProvider" da APi.

ApiPromise conforme acima como aplicado de fato na ES2015 para podermos utilizar as funões "async"/"await"

```js
// Import
import { ApiPromise, WsProvider } from '@polkadot/api';

// Construct / Criar objeto/instancia API
const wsProvider = new WsProvider('ws://localhost:9944');
const api = await ApiPromise.create({ provider: wsProvider });

console.log(api.consts.balances.transactionByteFee.toNumber());
console.log(api.genesisHash.toHex())

//version without <code>"async"</code> synthax
ApiPromise
  .create({ provider: wsProvider }).isReady
  .then((api) =>
    console.log(api.genesisHash.toHex())
  );

//version without ".create"

// Create the instance
const api = new ApiPromise({ provider: wsProvider });

// Wait until we are ready and connected
await api.isReady;

// Do something
console.log(api.genesisHash.toHex());
```

Com as operações acima realizadas, podemos acessar todas as funções e constantes da API para interagir com o node do Substrate.


"At this time the only provider type that is fully supported by the API is the WebSocket version. Polkadot/Substrate really comes alive with possibilities once you have access to bi-directional RPCs such as what WebSockets provide. (It is technically possible to have some limited capability via bare-HTTP, but at this point WebSockets is the only fully-operational and supported version - always remember that it is just "upgraded HTTP".)" 

Polkadot{.js}, https://polkadot.js.org/docs/api/start/create/



No caso da FLora, estamos importando no arquivo "index.js" e no "App.js"

```js
//index.js  
import {
  SubstrateContextProvider, useSubstrate
} from './SubstrateContext';
import utils from './utils';

export { useSubstrate, SubstrateContextProvider, utils };

//App.js
import { SubstrateContextProvider } from './substrate-lib';
export default function App () {
  return(
    <SubstrateContextProvider>
    //...
    </SubstrateContextProvider>);}
```

*** Lendo os Dados do Node (Queries)

Exemplo:
```js
// inicializando o objeto api
const api = ...;

// Obtendo o timestamp do node
const now = await api.query.timestamp.now();

// Pegando um endereço pré-configurado
const ADDR = '5DTestUPts3kjeXSTMyerHihn1uwMfLj8vU8sqF7qYrFabHE';

// Puxando o saldo do endereço
const balance = await api.query.balances.freeBalance(ADDR);
const nonce = await api.query.system.accountNonce(ADDR);

console.log(`${now}: balance of ${balance} and a nonce of ${nonce}`);

 ```

O método para ler dados do node é a partir do "api.query" e o nome após a consulta é contruído dinamicamente quando conectado ao node do Substrate. Dependendo de quais "pallets" são carregados, podemos  acessar o "storage" dos mesmos bem como a "getter function" correspondente. 

 Como preceito básico ocorre desta maneira:

 ```js
api.query ..;
 ```

Como a função acima realiza leitura em tempo real no node, esta operação será do tipo assìncrona, retornando uma "Promise", e sem seguida, usando "await" para aguardar o resultado.

"Subscription" - Um meio de se inscrever e receber mudanças de dados do node
Muitas vezes em se tratando de um front-end não basta apenas obter os dados do node no momento que a página da web é carregada, mas sim quado os dados na cadeia mudam, pois pode ser necessário alterar dinâmicamente o conteúdo da página.
É por esse motivo que, quando nos conectamos pela primeira vez ao node, não utilizamos uma simples solicitação http de API, mas uma conexão WebSocket.

Acima, no exemplo onde obtivemos o saldo do usuário, também podemos passar uma função de retorno de chamada. Desta forma, o saldo do usuário é mostrado, e toda vez que o valor do saldo for alterado, ele será reajustado.

 ```js
//unsub = desinscrito do valor
const unsub = await api.query.balances.freeBalance(ADDR, balance => {
  console.log(`balance of ${balance}`);
});
 ```

"To send a transaction and then waiting until it has been included in a block, we will use a subscription interface instead of just waiting for the transaction pool addition to yield the extrinsic hash. For the simplest form, we can do the following -"

 ```js
...
// Create alice (carry-over from the keyring section)
const alice = keyring.addFromUri('//Alice');

// Make a transfer from Alice to BOB, waiting for inclusion
const unsub = await api.tx.balances
  .transfer(BOB, 12345)
  .signAndSend(alice, (result) => {
    console.log(`Current status is ${result.status}`);

    if (result.status.isInBlock) {
      console.log(`Transaction included at blockHash ${result.status.asInBlock}`);
    } else if (result.status.isFinalized) {
      console.log(`Transaction finalized at blockHash ${result.status.asFinalized}`);
      unsub();
    }
  });
 ```

"As per all previous subscriptions, the transaction subscription returns in unsub() and the actual method has a subscription callback. The result object has 2 parts, events (to to covered in the next section) and the status enum.

When the status enum is in Finalized state (checked via isFinalized), the underlying value contains the block hash of the block where the transaction has been finalized. Finalized will follow InBlock, which is the block where the transaction has been included. InBlock does not mean the block is finalized, but rather applies to the transaction state, where Finalized means that the transaction cannot be forked off the chain."

A API JS também tem um método conveniente para se inscrever em vários valores do node de uma vez. Conforme:

 ```js
const unsub = await api.queryMulti([
  // Uma getter function
  api.query.timestamp.now,
  // Outra getter function，e os parâmetros necessários
  [api.query.balances.freeBalance, ADDR],
  [api.query.system.accountNonce, ADDR],
], ([now, balance, nonce]) => { // função de retorno de chamada
  console.log(`${now}: balance of ${balance} and a nonce of ${nonce}`);
});
 ```

Basta usar: api.queryMulti(queries ...//consultas, ...//função de retorno).

Constantes node consulta (Constant)
Esse método é semelhante à leitura de dados do node. Chama-se a interface da API, api.const.<pallet nome_do_pallet>.<pallet constate_do_pallet>.toNumber();

"Constant queries will introduce you to the concepts behind the types and the interaction of the API with those types. The same concepts are implemented in the remainder of the API - the runtime constants is just the simplest starting point.

For some background: constants are values that are defined in the runtime and used as part of chain operations. These constants can be changed as part of an upgrade.

Since these are constants and defined by the metadata, it is not a call, but rather the values immediately available - as you'll see in subsequent sections, there is no need for await on these, it immediately returns the type and value for you to work with."

// babe (constantes do pallet "babe" )
console.log(api.consts.babe.epochDuration.toNumber());

// balances (constantes do pallet "balances")
console.log(api.consts.balances.creationFee.toNumber());
console.log(api.consts.balances.transferFee.toNumber());


Nesse momento focaremos em alguns pontos: 
1º - as constantes são obtidas quando a API é conectada ao Node, portanto são retornadas diretamente e não precisam ser retornadas de forma assíncrona na forma de "Promise". 
2º - Embora a constante seja um número, a API JS nos ajuda a "empacotá-la" em um objeto ao retornar, e toNumber() é usado para obter um valor que possa ser reconhecido em JS a partir desse objeto. 

Transações Externas (Extrinsics) An extrinsic is a piece of information that comes from outside the chain and is included in a block
Esse tipo de operação altera diretamente os dados do Node/blockchain e requer que um usuário/alguém assine a chamada, por isso nomeou-se "extrinsic". Para contextualizar, no exemplo abaixo, a conta da Alice faz uma transação de depósito/pagamento de 12345 unidades de token para uma outra conta.

```js
// ...

// endereço fake/simulado de recebimento (Bob)
const recipient = '5DTestUPts3kjeXSTMyerHihn1uwMfLj8vU8sqF7qYrFabHE';

// Assina e envias a transferência de Alice para Bob
const txHash = await api.tx.balances
  .transfer(recipient, 12345)
  .signAndSend(alice);

// Exibe o código "hash" da transação
console.log(`Submitted with hash ${txHash}`);
```

O método de uso é:

```js
api.tx .. (parâmetro 1, ...);
```