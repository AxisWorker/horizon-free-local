# Horizon Free Local: instalar e jogar offline

## Primeira vez

1. Abra o launcher com internet.
2. Escolha um nome local no seletor de jogador. Sem escolha, será usado `Player`.
3. Crie uma nova instância **Vanilla**, selecione a versão do Minecraft Java e confirme.
4. Clique em **Jogar**. O launcher baixa os arquivos do jogo, bibliotecas, recursos
   e o Java necessário. Aguarde a instalação terminar; o jogo será iniciado.
5. Nas próximas vezes, essa instância completamente instalada pode abrir sem internet.

Não é necessário entrar em uma conta Microsoft no modo padrão. Uma instalação
interrompida pode ser retomada: os arquivos já verificados são reaproveitados.
O launcher não marca a instalação como completa se faltarem recursos.

## Minecraft Java e Horizon

A opção **Vanilla** executa o Minecraft Java na versão escolhida. **Horizon** é a
reimplementação em Rust e suporta a versão definida por cada build; não executa
arbitrariamente todas as versões antigas do Minecraft.

Fabric, Quilt, Forge e NeoForge usam o fluxo existente de instalação dos loaders.
A compatibilidade depende das versões escolhidas. Servidores que exigem
uma sessão autenticada continuam exigindo conta válida.

Nenhum arquivo do Minecraft acompanha estes downloads. Os arquivos necessários
são obtidos durante a instalação. O trabalho reduz o custo do launcher; não altera
os requisitos gráficos do jogo nem promete ganhos de FPS. Não foram feitos benchmarks.

## Terminal

```sh
horizon-launcher account offline Gui
horizon-launcher instance create "Vanilla 1.21.4" --minecraft 1.21.4
horizon-launcher play --instance "Vanilla 1.21.4"
```

No Windows, substitua `horizon-launcher` por `.\horizon-free-local-windows-x64.exe`.

## Opções

| Variável | Efeito |
| --- | --- |
| Sem variáveis | Nome local, downloads de instalação permitidos e interface leve |
| `HORIZON_LAUNCHER_OFFLINE=1` | Bloqueia downloads do launcher; precisa dos arquivos já instalados |
| `HORIZON_LAUNCHER_ONLINE=1` | Ativa o fluxo original de contas Microsoft e serviços online |
| `HORIZON_LAUNCHER_LIGHTWEIGHT=0` | Restaura os efeitos decorativos |

O bloqueio de downloads não é um firewall para jogos, mods ou programas externos.
`HORIZON_LAUNCHER_HOME` escolhe a pasta do launcher e `HORIZON_HOME`, a dos jogos.
Para preparar Horizon com arquivos locais, `HORIZON_SERVER_JAR` aceita o caminho
do seu `server.jar`, e `prepare --minecraft /caminho/minecraft` indica a pasta do jogo.

A interface leve mantém fundo simples, sem animação de abertura, parallax ou
imagens de desfoque, com menor frequência de consulta quando ociosa. Não há login
Microsoft, atualização automática, loja ou telemetria em segundo plano no modo local.
