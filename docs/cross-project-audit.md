# Cross-Project Reuse Audit — Mabel

> **Data:** 02/08/2026
> **Scope:** Todos os projetos em `C:\Users\LuisCarmo\Documents\Kiro\Projetos Pessoais`
> **Projeto principal:** mabel (Tauri v2 + React + Rust)

---

## Projetos analisados

| Projeto | Stack | Dispositivo | Transporte | Protocolo |
|---------|-------|-------------|-----------|-----------|
| **mabel** | Tauri v2, React 19, Rust (3 crates) | Soundcore Space One Pro (A3062) | RFCOMM (WinRT) | Soundcore (0x08/0x09 framing, nom) |
| **baseus-desktop** | Tauri v2, SolidJS, Rust (3 crates) | Baseus BP1 Pro ANC | BLE GATT (btleplug) | Baseus (0xAA magic, simples) |
| **OpenSCQ30** | GTK4, Rust (monorepo) | 20+ Soundcore devices incl. A3062 | RFCOMM (WinRT) | Soundcore (identico ao mabel) |
| **SoundcoreManager** | Tauri, TypeScript + Rust | Soundcore generico | BLE/RFCOMM (btleplug + WinRT) | Soundcore (checksum-based) |

---

## 1. Recursos ja reutilizados

| Recurso | Projeto de origem | Arquivo de origem | Arquivo de destino (mabel) | Tipo de reutilizacao | % aproveitamento | Status |
|---------|-------------------|-------------------|---------------------------|---------------------|-----------------|--------|
| Transport trait design (discover/connect/read/write) | baseus-desktop | `crates/baseus-transport/src/lib.rs` | `crates/mabel-transport/src/traits.rs` | Pattern rewrite (mesmo design, API adaptada para RFCOMM) | 80% | OK |
| MockTransport (rx_queue + tx_log) | baseus-desktop | `crates/baseus-transport/src/lib.rs` (MockTransport) | `crates/mabel-transport/src/mock.rs` | Pattern rewrite | 90% | OK |
| Windows device discovery (WinRT DeviceInformation) | OpenSCQ30 | `lib/src/connection_backend/windows/rfcomm.rs` (devices()) | `crates/mabel-transport/src/windows/rfcomm.rs` | Pattern rewrite (mesmo fluxo: AQS filter -> DeviceInformation -> BluetoothDevice -> MAC) | 85% | OK |
| A3062 state fields (byte layout) | OpenSCQ30 | `lib/src/devices/soundcore/a3062/state.rs` + packets | `crates/mabel-protocol/src/models/a3062/state.rs` | Data knowledge reuse (mesmos campos, struct simplificada) | 70% | OK |
| Soundcore packet wire format (direction headers, checksum) | OpenSCQ30 | `lib/src/devices/soundcore/common/packet.rs` | `crates/mabel-protocol/src/framing.rs` | Protocol knowledge (mesmos bytes 0x08/0x09, mesmo checksum) | 75% | OK |
| EQ presets (band values) | OpenSCQ30 | Test vectors + A3062 EQ config | `crates/mabel-protocol/src/models/a3062/parser.rs` (PRESET_BANDS) | Data extraction | 100% | OK |
| Tauri app shell (managed state + invoke_handler) | baseus-desktop | `apps/baseus-app/src-tauri/src/lib.rs` | `apps/mabel-app/src-tauri/src/lib.rs` | Pattern rewrite (mesma estrutura Builder) | 70% | OK |

---

## 2. Recursos que ainda podem ser reutilizados

| Recurso | Projeto de origem | Arquivo | Motivo | Beneficios | Complexidade | Riscos |
|---------|-------------------|---------|--------|-----------|--------------|--------|
| **RFCOMM connect() completo** (StreamSocket + read channel + connection status events) | OpenSCQ30 | `connection_backend/windows/rfcomm.rs` | Mabel tem `connect()` como TODO stub. O OpenSCQ30 tem implementacao production-grade | Semanas de trabalho poupadas, battle-tested em 20+ devices | Media (adaptar de `macaddr::MacAddr6` para `String`, remover features extras) | Baixo |
| **Device loop pattern** (tokio::select! + reconnect + command channel + event emission) | baseus-desktop | `apps/baseus-app/src-tauri/src/device.rs` | Mabel ainda nao tem device loop no Tauri. O baseus tem um padrao provado com retry, timeout detection, command dispatch | Arquitectura robusta day-1, reconnection automatica | Baixa (copiar pattern, adaptar commands para Soundcore) | Nenhum |
| **Tauri IPC bridge** (typed invoke wrappers + event listeners) | baseus-desktop | `apps/baseus-app/src/lib/tauri.ts` | Mabel usa dynamic `import()` inline nos hooks. O baseus tem um modulo centralizado com tipos | Type safety end-to-end, single import point, easier to swap mocks | Baixa (criar `src/lib/tauri.ts` com wrappers tipados) | Nenhum |
| **Notification/battery alerts** (threshold-based, OS-level) | baseus-desktop | `src-tauri/src/device.rs` | Mabel mostra bateria na UI mas nao alerta quando esta baixa | UX premium, user nao precisa olhar para o app | Baixa (usar `tauri-plugin-notification`) | Nenhum |
| **System tray** (minimize to tray, tray icon) | baseus-desktop | `src-tauri/src/tray.rs` | Mabel nao tem tray icon. Uma companion app deve ficar em background | App permanece acessivel sem janela aberta | Baixa | Nenhum |
| **Auto-reconnection logic** | baseus-desktop | `device.rs` loop externo com `RETRY_DELAY` | Mabel perde conexao e nao reconecta | Experiencia seamless | Baixa (loop infinito com sleep) | Nenhum |
| **Settings persistence** (JSON file) | baseus-desktop | `src-tauri/src/settings.rs` | Mabel nao persiste preferencias do utilizador | Manter preferencias entre sessoes | Baixa | Nenhum |

---

## 3. Codigo duplicado

| Tipo | Mabel | Outro projeto | Grau de duplicacao | Notas |
|------|-------|---------------|-------------------|-------|
| Windows BT device discovery | `mabel-transport/src/windows/rfcomm.rs` | `OpenSCQ30/connection_backend/windows/rfcomm.rs` | **90% logica identica** | Mesmo AQS filter, mesmo DeviceInformation flow, mesmo format_mac. OpenSCQ30 e mais robusto (MAC filter). |
| Transport error enum | `mabel-transport/src/error.rs` (6 variants) | `baseus-transport/src/lib.rs` (5 variants) | **80% equivalente** | Mesmas categorias: ConnectionFailed, DeviceNotFound, Disconnected, Io/Platform |
| Packet checksum | `mabel-protocol/framing.rs` compute_checksum() | `SoundcoreManager/soundcore-lib/src/parsers.rs` generate_checksum() | **100% identico** | Ambos: sum of bytes mod 256, fold com wrapping_add |
| ConnectionDescriptor struct | `mabel-transport/traits.rs` | `OpenSCQ30/api/connection.rs` | **95% equivalente** | Ambos: name + mac_address. OpenSCQ30 usa `MacAddr6`, mabel usa `String` |
| ConnectionStatus enum | `mabel-transport/traits.rs` | `OpenSCQ30/api/connection.rs` + baseus (via events) | **100% equivalente** | Connected/Disconnected em todos |

---

## 4. Oportunidades de centralizacao

| O que centralizar | Tipo sugerido | Projectos que beneficiam | Esforco | Prioridade |
|-------------------|---------------|------------------------|---------|-----------|
| **Soundcore packet framing** (checksum + header) | Crate compartilhado `soundcore-wire` | mabel + SoundcoreManager | Alto (2 projetos teriam que migrar) | Baixa |
| **Windows Bluetooth discovery** (AQS filter + DeviceInformation) | Modulo util em `mabel-transport` (ja esta correcto) | Apenas mabel | N/A | Ja esta no lugar certo |
| **Tauri IPC bridge pattern** | Template/doc em `docs/patterns/` | mabel + baseus-desktop | Baixo (e um padrao, nao uma lib) | Media |
| **Device loop + reconnect** | Padrao documentado + codigo template | mabel + baseus-desktop | Baixo | Alta |

**Conclusao:** Dado que os projetos usam tecnologias diferentes (BLE vs RFCOMM, Soundcore vs Baseus protocol, React vs SolidJS), a centralizacao em bibliotecas partilhadas **nao e recomendada**. O custo de manutencao excede o beneficio. O correcto e **reutilizar patterns** (design) e **dados de protocolo** (byte layouts), nao criar dependencias cruzadas.

---

## 5. Percentual de reaproveitamento

| Metrica | Valor | Notas |
|---------|-------|-------|
| **Codigo reutilizado (patterns aplicados)** | ~35% | Transport trait, mock, discovery, state struct, wire format |
| **Codigo novo (original do mabel)** | ~55% | React UI, A3062 parser implementation, Tauri commands, custom hooks |
| **Codigo duplicado** | ~8% | Discovery logic, checksum, error enums |
| **Codigo que pode ser reaproveitado** | ~12% | connect(), device loop, IPC bridge, tray, settings, alerts |
| **% potencial apos melhorias** | ~47% reutilizado | Subindo de 35% para 47% |

---

## 6. Plano de migracao

### Etapa 1 — Importar imediatamente (proximas sessoes)

| # | Accao | Fonte | Destino | Impacto |
|---|-------|-------|---------|---------|
| 1 | Criar `src/lib/tauri.ts` com invoke wrappers tipados | baseus `src/lib/tauri.ts` | mabel `src/lib/tauri.ts` | Type-safe IPC, remove dynamic imports dos hooks |
| 2 | Adoptar device loop pattern para Phase D | baseus `device.rs` run_loop() | mabel `src-tauri/src/device.rs` | Reconnection automatica, command dispatch |

### Etapa 2 — Reutilizar com adaptacoes (Phase B3)

| # | Accao | Fonte | Adaptacao necessaria |
|---|-------|-------|---------------------|
| 3 | Implementar `connect()` baseado em OpenSCQ30 | `WindowsRfcommConnection` | Simplificar (remover service selection strategy, usar SPP UUID fixo) |
| 4 | Implementar read channel (thread + mpsc) | `spawn_read_channel()` do OpenSCQ30 | Adaptar de `AgileReference` pattern (ja usado na discovery) |
| 5 | Implementar connection status (watch channel + event handler) | OpenSCQ30 `ConnectionStatusChanged` | Direct copy do pattern |

### Etapa 3 — Funcionalidades adicionais (Phase E + polish)

| # | Accao | Fonte | Notas |
|---|-------|-------|-------|
| 6 | System tray | baseus `tray.rs` | Plugin `tauri-plugin-tray` |
| 7 | Settings persistence | baseus `settings.rs` | JSON file no config dir |
| 8 | Battery alerts | baseus `maybe_alert_battery()` | `tauri-plugin-notification` |
| 9 | Auto-updater | baseus `commands.rs` check_update | `tauri-plugin-updater` |

### Etapa 4 — Permanecer exclusivo do mabel

| Item | Razao |
|------|-------|
| React UI (todas as pages/components) | baseus e SolidJS, OpenSCQ30 e GTK — incompativel |
| A3062 state parser byte offsets | Especifico do model, derivado de reverse engineering |
| Framer Motion animations | Especifico do frontend React |
| EQ visualizer component | Custom SVG, sem equivalente noutros projetos |
| Radix UI primitives (Slider/Switch/Select) | React ecosystem, nao existe em SolidJS |
| Custom title bar | Implementacao React-specific |

### Ordem ideal de migracao

```
Etapa 1 (imediato): IPC bridge + device loop pattern
Etapa 2 (Phase B3): connect() + read channel + status
Etapa 3 (Phase E): tray + settings + alerts + updater
Etapa 4: manter exclusivo (frontend, parser, animations)
```

---

## 7. Resultado final

| Recurso | Projeto de origem | Situacao actual | Sera reutilizado | Accao necessaria |
|---------|-------------------|----------------|-----------------|-----------------|
| Transport trait (discover/connect/read/write) | baseus-desktop | Reutilizado | Sim | Nenhuma |
| MockTransport (rx_queue + tx_log) | baseus-desktop | Reutilizado | Sim | Nenhuma |
| Windows discovery (WinRT) | OpenSCQ30 | Reutilizado | Sim | Nenhuma |
| A3062 state fields | OpenSCQ30 | Reutilizado | Sim | Nenhuma |
| Soundcore wire format | OpenSCQ30 | Reutilizado | Sim | Nenhuma |
| EQ presets data | OpenSCQ30 | Reutilizado | Sim | Nenhuma |
| Tauri shell pattern | baseus-desktop | Reutilizado | Sim | Nenhuma |
| RFCOMM connect() | OpenSCQ30 | TODO stub | **Sim** | Implementar baseado em OpenSCQ30 |
| Device loop (select! + reconnect) | baseus-desktop | Nao existe | **Sim** | Copiar pattern em Phase D |
| Typed IPC bridge | baseus-desktop | Usa dynamic import | **Sim** | Criar `src/lib/tauri.ts` |
| System tray | baseus-desktop | Nao existe | **Sim** | Adicionar em Phase E |
| Settings persistence | baseus-desktop | Nao existe | **Sim** | Adicionar em Phase E |
| Battery alerts | baseus-desktop | Nao existe | **Sim** | Adicionar em Phase E |
| Auto-updater | baseus-desktop | Nao existe | **Sim** | Adicionar em Phase E |
| React UI completa | Original | Original | Exclusivo | Nenhuma |
| A3062 parser (byte offsets) | Original (dados OpenSCQ30) | Original | Exclusivo | Corrigir offsets |
| Framer Motion animations | Original | Original | Exclusivo | Nenhuma |
| EQ visualizer | Original | Original | Exclusivo | Nenhuma |
| Custom title bar | Original | Original | Exclusivo | Nenhuma |

---

## Resumo executivo

### O que ja foi reutilizado
- **7 patterns** do baseus-desktop e OpenSCQ30: transport trait design, mock transport, Windows discovery, A3062 state fields, Soundcore wire format, EQ presets, Tauri shell pattern.
- Tipo: **pattern rewrite** (nao copy-paste) — o design e identico mas o codigo e escrito de novo, mais simples e sem bagagem.

### O que ainda sera reutilizado
- **7 recursos** pendentes: RFCOMM connect(), device loop, IPC bridge, system tray, settings, battery alerts, auto-updater.
- Timing: Phase B3 (connect), Phase D (device loop), Phase E (tray + settings + alerts + updater).

### O que permanecera exclusivo
- **100% do frontend React** (5 pages, 15+ components, hooks, providers, animations).
- **A3062 parser implementation** (byte-level parsing proprio, derivado de dados publicos do OpenSCQ30).
- **Custom UX** (animations, transitions, dark theme system).

### Percentual estimado de reaproveitamento apos melhorias
- **Actual:** ~35% (patterns aplicados)
- **Apos completar etapas 1-3:** ~47%
- **Codigo genuinamente novo:** ~53% (frontend + parser + UX polish)

### Veredicto
O projecto esta a **reutilizar correctamente** — aplica patterns provados sem criar dependencias frageis. As oportunidades restantes (connect, device loop, IPC bridge) estao planeadas para as fases seguintes. Nao ha codigo a ser reinventado desnecessariamente.
