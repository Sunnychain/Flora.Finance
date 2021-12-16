## Documentação gerada usando o Retype (https://github.com/retypeapp/retype)

Para instalar: `npm`, `yarn`, ou `dotnet` CLI.

From your command line, navigate to a folder location where you have one or more Markdown `.md` files, such as a GitHub project.

-- Comandos importantes:

+++ NPM
```
npm install retypeapp --global
retype watch
```
+++ Yarn
```
yarn global add retypeapp
retype watch
```
+++ dotnet
```
dotnet tool install retypeapp --global
retype watch
```
+++

---

## Live reload

Se alguma mudança for detectada, como alterações nos arquivos .md e ações de salvar, o Retype fará o update instantaneamente no browser.

O comando `retype watch` exexuta os três comandos abaixo, e monitora as mudanças em tempo real nos arquivos .md

```
retype init
retype build
retype run
```

------------------------------------------------------------
