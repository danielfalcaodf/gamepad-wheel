# PRD — Celular como volante + GameSir Nova 2 Lite

Nov 10, 2026 · @Daniel Falcão

## Resumo e problema

Um programa para Windows, em Rust, junta o giroscópio de um celular Android e o GameSir Nova 2 Lite em um único controle de Xbox virtual. O celular, preso ao controle por um suporte, vira o volante, e o Nova 2 Lite cuida de acelerar, frear, câmera, setas, câmbio e o resto.

**Problema.** O Nova 2 Lite não tem giroscópio, então não dá para virar o carro inclinando o controle. Os apps prontos de celular-volante são pagos ou mostram anúncios a cada 20 minutos, e não juntam o celular com um controle físico. Os jogos costumam aceitar um controle só, então usar dois dispositivos ao mesmo tempo não funciona sem um programa no meio.

**Para quem.** Para o próprio autor, em um PC Windows com celular Android, jogando ETS2/ATS, Forza/NFS, Assetto Corsa/BeamNG/F1 e GTA. O projeto é pessoal, mas organizado para ser publicado como open source depois.

## Objetivos e não-objetivos

**Objetivos**

1. Jogar os quatro grupos de jogos com o celular como volante e o Nova 2 Lite para o resto, sem pagar nada nem ver anúncios.
2. Fazer o jogo enxergar um único controle de Xbox, com o Nova 2 Lite físico escondido automaticamente.
3. Ter direção responsiva: latência ponta a ponta (celular → jogo) baixa o bastante para não ser percebida em curva (meta em Métricas).
4. Não exigir braços cruzados: no máximo cerca de ±90° de giro do celular, com o analógico completando o resto quando o perfil pedir.
5. Trocar de perfil sozinho ao abrir cada jogo.

**Não-objetivos (v1)**

- **iOS.** O app é Kotlin nativo. O protocolo fica documentado para um cliente iOS no futuro.
- **Linux/macOS no PC.** ViGEmBus e HidHide só existem no Windows.
- **USB entre celular e PC.** A v1 é Wi-Fi UDP; USB (via adb) entra como opção futura.
- **Telemetria do jogo no celular** (velocidade, marcha, RPM). Cada jogo tem uma API diferente, o que aumenta muito o escopo.
- **Force feedback de volante de verdade.** A saída é um controle Xbox; a vibração vai só para o Nova 2 Lite.

## Arquitetura e decisões técnicas

O jogo só vê o Xbox virtual e o teclado simulado; o Nova 2 Lite físico fica escondido pelo HidHide, e o nosso programa é o único que o lê.

&#91;embedded content: arquitetura · celular, controle, programa no PC, jogo\]

O mixer aplica o perfil do jogo aberto: combina celular e stick na direção e transforma combos e botões da tela em botões Xbox ou teclas.

| Decisão | Escolha | Motivo |
| --- | --- | --- |
| Linguagem do PC | Rust | um .exe pequeno, sem dependências, latência estável; o autor quer aprender Rust |
| Controle virtual | ViGEmBus, crate `vigem-client` | o jogo enxerga um Xbox 360 comum, sem configurar nada no jogo |
| Esconder o controle físico | HidHide, ligado e desligado pelo programa | o jogo não vê dois controles |
| Interface gráfica | egui (a confirmar na Q7) | Rust puro, binário único, bom para teste ao vivo |
| Perfis | arquivos TOML editados pela GUI | legíveis, versionáveis no Git, fáceis de compartilhar depois |
| App do celular | Kotlin nativo + Compose | acesso direto ao sensor de rotação fundido, menor latência |
| Transporte | UDP no Wi-Fi, protocolo binário próprio e versionado | simples e rápido; o mesmo protocolo serve para USB e iOS no futuro |
| Pareamento | QR code com IP, porta e token | não digita IP e ignora pacotes de fora |

## Histórias de usuário

**Dirigindo**

- Como jogador, quero virar o carro inclinando o controle com o celular preso, para ter a sensação de volante sem comprar um.
- Como jogador, quero acelerar e frear nos gatilhos Hall Effect do Nova 2 Lite, para ter dosagem analógica fina.
- Como jogador de ETS2/ATS, quero dar seta, ligar farol, pisca-alerta e trocar marcha por combos no controle ou por botões na tela do celular, para não precisar do teclado.
- Como jogador, quero girar só até uns 90° e completar a curva com o analógico, para não jogar de braços cruzados.
- Como jogador de GTA, quero que o analógico esquerdo continue funcionando quando eu sair do carro, para andar a pé sem trocar de modo.

**Configurando**

- Como jogador, quero escanear um QR code na tela do PC e já estar conectado, para não digitar IP.
- Como jogador, quero que o perfil certo carregue sozinho quando abro o jogo, para não lembrar de trocar.
- Como jogador, quero ajustar ângulo, curva, zona morta e modo de soma em uma interface gráfica com teste ao vivo, para calibrar sem editar arquivo.
- Como jogador, quero recentralizar o volante com um botão no controle ou na tela, para corrigir quando o celular sair do centro.

**Quando algo dá errado**

- Como jogador, quero que a direção volte ao centro e o controle vibre se o Wi-Fi cair, para o carro não ficar virado sozinho.
- Como jogador, quero ver a latência e o status da conexão na tela do celular, para saber se o problema é a rede.
- Como jogador, quero que o Nova 2 Lite volte a aparecer para os jogos quando eu fechar o programa, para usar o controle normalmente depois.

## Requisitos

A v1 inclui tudo o que é P0 e P1. P0 é o primeiro marco jogável; P1 completa a v1. P2 fica fora, mas a arquitetura não deve impedir.

### P0: primeiro marco jogável

**R1. Envio do ângulo pelo celular.** O app lê `TYPE_GAME_ROTATION_VECTOR`, calcula o ângulo de rolagem com o celular em paisagem e envia por UDP a 100–200 Hz.

- [ ] Pacote com versão do protocolo, número de sequência, timestamp, ângulo e botões da tela.
- [ ] O ângulo não salta de +180° para −180° (ângulo acumulado, não cortado).
- [ ] A tela não apaga enquanto conectado.

**R2. Pareamento por QR code.** O PC mostra um QR com IP, porta e um token de sessão; o app escaneia e conecta.

- [ ] Pacotes sem o token da sessão são descartados.
- [ ] Com mais de uma placa de rede, o PC escolhe o IP da rede local, e é possível trocar na GUI.

**R3. Leitura do Nova 2 Lite.** O PC lê sticks, gatilhos e botões do controle no modo XBOX (dongle ou cabo).

- [ ] Gatilhos chegam como valor analógico, não liga/desliga.
- [ ] Se o controle desconectar, a GUI avisa e o controle virtual mantém o celular funcionando.

**R4. Controle Xbox virtual.** O PC cria um Xbox 360 virtual via ViGEmBus e junta as entradas.

- [ ] Analógico esquerdo X = direção combinada (R5).
- [ ] Gatilhos, analógico direito (câmera), D-pad e botões vêm do Nova 2 Lite.
- [ ] Ciclo de saída de pelo menos 250 Hz.

**R5. Combinação da direção.** Por perfil: ângulo máximo do celular, zona morta, curva e um modo de soma:

1. **Celular base + stick extra.** ±N° do celular = X% do volante; o stick no mesmo lado completa até 100%.
2. **Celular 100% + stick fino.** ±N° vira tudo; o stick corrige.
3. **Curva não linear.** Centro suave, ponta mais forte, sem depender do stick.

Em todos os modos, se o stick esquerdo passar de um limiar, ele assume a direção, e o celular volta quando o stick é solto (para andar a pé no GTA).

- [ ] Dado celular a +45° com máximo de 90° no modo 1 (X = 70%), então a saída fica perto de +35%.
- [ ] A saída nunca passa de ±100%.

**R6. Recentralizar.** Um botão no Nova 2 Lite (configurável) e um botão na tela do app zeram o ângulo atual.

**R7. Perda de conexão.** Mais de 100 ms sem pacote do celular → direção vinda do celular vai a 0, o Nova 2 Lite dá uma vibração curta e a GUI mostra o aviso.

- [ ] O stick esquerdo continua dirigindo durante a perda.
- [ ] Ao voltar, a direção retoma suavemente (sem salto), em até 200 ms.

**R8. HidHide automático.** Ao iniciar, o programa se coloca na lista de permitidos do HidHide e esconde o Nova 2 Lite; ao fechar, ou se travar, o controle volta a aparecer.

- [ ] Se HidHide ou ViGEmBus não estiverem instalados, a GUI diz qual falta e como instalar.
- [ ] O jogo vê um único controle (verificar em joy.cpl e no próprio jogo).

### P1: completa a v1

**R9. Interface gráfica.** Janela com status (celular, controle, HidHide, perfil ativo), editor de perfis e um teste ao vivo mostrando ângulo, saída do volante e gatilhos. Ícone na bandeja.

**R10. Perfis automáticos.** Cada perfil tem uma lista de executáveis; o perfil troca quando o jogo em primeiro plano muda. Há um perfil padrão para jogos sem perfil.

**R11. Camada de combos.** Segurar o botão de camada (L4/R4, ver Perguntas em aberto) + outro botão gera uma ação. Uma ação pode ser um botão Xbox **ou uma tecla do teclado**, porque o Xbox tem poucos botões e jogos como ETS2 usam teclas para seta e farol.

- [ ] Enquanto a camada está ativa, o botão original não chega ao jogo.
- [ ] Ações de toque, segurar e alternar (liga/desliga).

**R12. Botões na tela do celular.** Botões grandes e configuráveis por perfil (seta, farol, pisca-alerta, câmbio), com a mesma saída de R11.

**R13. Tela do app.** Indicador de ângulo, saída do volante, latência (ms) e status. Modo de tela escura para economizar bateria.

**R14. Vibração.** O rumble que o jogo manda ao Xbox virtual é repassado aos motores do Nova 2 Lite.

### P2: futuro

- **R15.** Conexão por USB (adb reverse), usando o mesmo protocolo.
- **R16.** Cliente iOS, usando o mesmo protocolo.
- **R17.** Publicar como open source: README, instalador, licença.
- **R18.** Telemetria do jogo na tela do celular.

## Métricas de sucesso

São metas de projeto pessoal, medidas pelo próprio programa (log na GUI) em Wi-Fi 5 GHz. Os valores são hipóteses iniciais, a revisar depois das primeiras sessões.

| Métrica | Como medir | Meta | Ideal |
| --- | --- | --- | --- |
| Latência celular → controle virtual (p95) | timestamp do pacote × hora de saída no PC, com relógios sincronizados no pareamento | < 30 ms | < 15 ms |
| Pacotes perdidos | lacunas no número de sequência | < 1% | < 0,1% |
| Quedas de conexão (> 100 ms sem pacote) | contador por sessão | < 1 por hora | 0 |
| Tempo de abrir o programa até dirigir | cronometrado, já pareado antes | < 30 s | < 10 s |
| Jogos-alvo funcionando com um único controle visível | checklist dos 4 grupos | 4 de 4 | 4 de 4 sem ajuste manual no jogo |
| Sessão longa | 1 h de ETS2 sem reiniciar nada | sem travar | bateria do celular > 50% no modo tela escura |

O critério que mais importa: jogar uma hora sem perceber atraso na direção e sem voltar ao teclado para seta e farol.

## Perguntas em aberto e riscos

Todas se respondem com um teste rápido (spike) no seu PC antes de escrever o requisito afetado.

| # | Pergunta | Bloqueia | Como responder | Plano B |
| --- | --- | --- | --- | --- |
| Q1 | Com o HidHide escondendo o Nova 2 Lite, o nosso programa (na lista de permitidos) ainda consegue lê-lo via XInput? | R3, R8 | spike: esconder no HidHide e ler com o programa permitido | ler via HID bruto/SDL em vez de XInput |
| Q2 | No modo XBOX, os botões traseiros L4/R4 só copiam um botão existente, então o PC não os distingue. Qual botão vira a camada? | R11 | testar o que o PC recebe ao apertar L4/R4 | usar View (Back) como camada, ou ler o controle em outro modo que exponha botões extras |
| Q3 | O Nova 2 Lite no dongle aceita vibração enviada pelo PC (XInputSetState)? | R7, R14 | spike com um teste de rumble | aviso só na GUI e no celular |
| Q4 | O ViGEmBus ainda é mantido e funciona no Windows atual? | R4 | conferir o repositório e instalar | outro driver de controle virtual (avaliar) |
| Q5 | Teclas enviadas pelo programa (SendInput) chegam a ETS2, Forza, BeamNG e GTA? | R11, R12 | testar seta/farol em cada jogo | usar códigos de varredura (scan codes) em vez de teclas virtuais |
| Q6 | Algum jogo com modo online/anti-cheat bloqueia controle virtual ou teclas simuladas? | jogos-alvo | testar GTA e Forza online | usar só no modo offline |
| Q7 | GUI em egui (Rust puro) ou Tauri (web + Rust)? | R9 | protótipo de uma tela em cada | egui por padrão (binário único) |

Riscos que não bloqueiam: o firewall do Windows vai pedir permissão na primeira vez (porta UDP); o suporte barato pode deixar o celular solto e gerar tremida no ângulo (zona morta e filtro no R5 compensam); Wi-Fi 2,4 GHz congestionado aumenta a latência (a GUI deve mostrar isso).

## Fases e marcos

Sem prazo: é hobby. A ordem importa mais que as datas, porque os testes da fase 0 decidem como R3, R8 e R11 serão feitos.

&#91;embedded content: fases · 4 fases, 3 marcos, sem datas (hobby)\]

Cada marco é o critério para passar de fase. A fase 1 já é jogável no Forza com os botões repassados direto; os combos e a GUI chegam na fase 2.
