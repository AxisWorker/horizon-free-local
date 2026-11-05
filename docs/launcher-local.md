# Launcher local e leve

Esta alteração torna o launcher local e econômico por padrão. Não exige conta
Microsoft para iniciar uma versão Horizon instalada e preparada no computador.

## O que muda

- Fundo simples, sem a cena animada, parallax ou imagens de desfoque dos painéis.
- Abertura imediata, sem a animação inicial.
- Consulta de resultados em segundo plano a cada 500 ms quando ocioso e 100 ms
  durante preparação, instalação ou início do jogo. Teclado e mouse continuam
  sendo tratados diretamente pelos eventos da interface.
- Sem varrer e completar os recursos de todas as versões ao abrir. A versão
  escolhida ainda recebe os arquivos locais necessários quando é preparada ou iniciada.
- Sem consulta automática de atualizações, links remotos, loja ou contagens online.
- Sem acessar o chaveiro ou renovar uma sessão Microsoft para jogar localmente.
- Sem enviar a telemetria do launcher; o processo Horizon recebe
  `HORIZON_TELEMETRY_URL=off` e não recebe sessões de outras contas.
- Minimiza ao iniciar o jogo quando ainda não existe uma preferência salva.

As contas existentes continuam no arquivo de contas. No modo local, a seleção de
conta Microsoft é ignorada em memória; o nome offline escolhido continua disponível.
O comando `account offline NOME` permite escolher esse nome sem fazer login.

## Usar

Com uma versão já instalada e preparada, abra o launcher normalmente ou use:

```sh
horizon-launcher account offline Gui
horizon-launcher play
```

O comando `play` usa o caminho de terminal existente e inicia sem criar a janela
GPUI. É a opção com menos trabalho gráfico para o launcher. Ele acompanha o
processo para manter o registro de tempo e os comandos configurados para depois do jogo.

Sem uma versão instalada, copie a pasta completa de uma instalação Horizon para
`<pasta do launcher>/versions/<versão>`. Ela deve conter o executável, o marcador
`.installed`, `research-lock.json` e os recursos/preparador correspondentes.
`HORIZON_LAUNCHER_HOME` permite escolher a pasta do launcher; `HORIZON_HOME`
escolhe a pasta de mundos e configurações do jogo.

Se os recursos ainda precisarem ser preparados, use seus arquivos locais da
versão correta do Minecraft, o Java correspondente e seu `server.jar`:

```sh
HORIZON_SERVER_JAR=/caminho/completo/server.jar \
  horizon-launcher prepare --minecraft /caminho/minecraft
```

No PowerShell:

```powershell
$env:HORIZON_SERVER_JAR = 'C:\Minecraft\server.jar'
.\horizon-launcher.exe prepare --minecraft 'C:\Minecraft'
```

Arquivos ausentes são informados como erro. O modo local não baixa o jogo, seus
recursos nem o Java. Nenhum arquivo do Minecraft acompanha esta alteração.
As verificações de versão e integridade dos recursos continuam no preparador.

## Recursos online opcionais

São escolhas independentes, aplicadas ao reiniciar o programa:

| Variável | Comportamento |
| --- | --- |
| Sem variáveis | Local e leve |
| `HORIZON_LAUNCHER_ONLINE=1` | Ativa os serviços online existentes; jogar offline continua disponível |
| `HORIZON_LAUNCHER_LIGHTWEIGHT=0` | Restaura a cena e os efeitos visuais existentes |

O modo local controla os caminhos automáticos alterados do launcher e bloqueia
seus downloads compartilhados. Não é um firewall para mods, programas externos,
uma instância Java, ou conexões multiplayer que o jogador abrir explicitamente.
Servidores que exigem autenticação continuam exigindo uma conta válida.

## Compilar

Na raiz do repositório, com Rust e as dependências gráficas da plataforma:

```sh
cargo build --release --manifest-path launcher/Cargo.toml
```

O executável fica em `launcher/target/release/`. Os testes não substituem uma
medição em PC fraco: ganhos de RAM, CPU e tempo de abertura ainda devem ser
medidos no mesmo hardware, com as mesmas versões instaladas e configurações.
