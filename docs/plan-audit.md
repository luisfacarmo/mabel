# Plan Audit — Mabel Implementation Review

## Audit Criteria

1. **Reaproveitamento** — Estamos a reutilizar o máximo das arquitecturas existentes?
2. **Quick wins** — Estamos focados em funcionalidade rápida?
3. **Monolitos** — Estamos a criar blocos demasiado grandes?
4. **Reinvenção da roda** — Estamos a refazer algo que já existe?
5. **Originalidade** — Estamos a criar, não apenas copiar?

---

## 1. Reaproveitamento das Arquitecturas Existentes

### O que o `baseus-desktop` já resolve e podemos reutilizar:

| Pattern | baseus-desktop | Mabel plan | Veredicto |
|---------|---------------|------------|-----------|
| Transport trait (connect/send/recv) | `BluetoothTransport` trait + `MockTransport` | Planeamos criar do zero | ✅ **Reaproveitar o design pattern** — trait idêntico, mock incluído |
| Tauri `lib.rs` setup | Plugins + managed state + spawn device loop | Planeamos criar do zero | ✅ **Copiar o skeleton** — é boilerplate, não lógica |
| `tauri.ts` bridge (frontend) | Typed invoke wrappers + event listeners | Planeamos criar do zero | ✅ **Reaproveitar a estrutura** — adaptar types |
| Device loop pattern | `run_loop()` com reconnect + select! | Plan D4 "reconnection logic" | ✅ **Reaproveitar o pattern** (select! + retry) |
| Command channel pattern | `mpsc::unbounded_channel` + enum dispatch | Plan D3 | ✅ **Reaproveitar** — pattern é proven |
| Sidebar component | icon rail + active marker | Plan A3 | ⚠️ **Inspirar, não copiar** — SolidJS → React, redesign |
| Battery ring SVG | `useCountUp` + SVG circle | Plan A5 | ⚠️ **Inspirar** — converter para React + Framer Motion |
| ANC tab layout | Mode cards + slider | Plan A6 | ⚠️ **Inspirar** — adaptar para 3 modes + sub-modes |

### O que o `OpenSCQ30` já resolve:

| Pattern | OpenSCQ30 | Mabel plan | Veredicto |
|---------|-----------|------------|-----------|
| RFCOMM Windows backend | Production-ready `WindowsRfcommBackend` | Plan B2-B3 | ❌ **NÃO podemos usar diretamente** — é lib interna, não publicada no crates.io. Mas o **design é a referência**. |
| Packet framing | `nom` parsers, checksum, direction headers | Plan C1 | ✅ **Reaproveitar o formato do protocolo** — é reverse engineering, não código |
| A3062 state parser | Exact byte offsets + test vector | Plan C3 | ✅ **Reaproveitar os dados** (byte layout, test vector) — mas escrever parser próprio, mais simples |
| `soundcore_device!` macro | Complex macro for device registration | Plan D1-D2 | ❌ **NÃO copiar** — over-engineering para 1 device. Nosso é directo. |

### Veredicto: ✅ Bom

Estamos a reaproveitar os **patterns** (trait design, device loop, command channel, IPC bridge) sem copiar código verbatim. Os dados do protocolo (byte layout, checksums) são reutilizados correctamente — são factos, não código.

---

## 2. Quick Wins — Estamos focados em funcionalidade rápida?

### Problemas identificados:

| Issue | Onde | Fix |
|-------|------|-----|
| ⚠️ Task A7 (EQ) é complexa demais para uma primeira iteração | Phase A | **Simplificar:** v1 = preset chips + static bars. Drag interaction = Task A7b separada. |
| ⚠️ Task A10 (polish) mistura muitas coisas | Phase A | **Split:** window chrome (A10a) vs animations (A10b) vs keyboard shortcuts (A10c) |
| ⚠️ Phase C5 (streaming parser) é prematura | Phase C | **Adiar:** O buffer do RFCOMM Windows já entrega pacotes completos (1 notificação = 1 pacote em 99% dos casos). Adicionar streaming só se falhar na prática. |
| ✅ Frontend-first é o quick win correcto | Order | O utilizador vê resultado em 1-2 sessões |

### Sugestão de reordenamento dentro da Phase A:

```
A1: Scaffold (obrigatório)
A2: Design tokens (obrigatório)
A3: Layout + sidebar (obrigatório)
A4: Mock data (obrigatório)
A5: Home page (QUICK WIN — visual impact imediato)
--- milestone: "app looks real" ---
A6: ANC page
A8: Controls page (simples, rápido)
A9: Settings page (toggles, rápido)
A7: EQ page (a mais complexa, por último)
A10: Polish (só depois de tudo funcionar)
```

---

## 3. Monolitos — Blocos demasiado grandes?

### Análise:

| Componente | Tamanho | Veredicto |
|-----------|---------|-----------|
| `mabel-transport` crate | ~300 LOC estimado | ✅ Correcto — faz uma coisa |
| `mabel-protocol` crate | ~500 LOC estimado | ✅ Correcto — faz uma coisa |
| `mabel-core` crate | ~400 LOC estimado | ⚠️ Potencial monolito |
| Frontend `useDeviceState` hook | Tudo num context | ⚠️ Risco de God Object |

### Fixes sugeridos:

1. **`mabel-core` não deve conter o device loop do Tauri.** O device loop (select!, reconnect) deve ficar no `src-tauri/` do Tauri app. `mabel-core` só expõe:
   - `DeviceManager::new(transport)`
   - `DeviceManager::connect() / disconnect()`
   - `DeviceManager::state() -> watch::Receiver`
   - `DeviceManager::send_command(Command)`

   O loop vive no app, não no crate. **Isto é o que o baseus-desktop faz** (`device.rs` no Tauri, não num crate separado).

2. **Frontend: separar state em hooks menores** em vez de um God Context:
   - `useConnection()` — connected/disconnected/reconnecting
   - `useDeviceState()` — readonly device state (from events)
   - `useCommands()` — setter functions (invoke Tauri)

---

## 4. Reinvenção da Roda

### O que estamos a reinventar desnecessariamente:

| Item | Existe já | Sugestão |
|------|-----------|----------|
| ❌ Custom slider component | Radix UI Slider, ou `react-aria` | Usar `@radix-ui/react-slider` com styling custom |
| ❌ Custom toggle switch | Radix UI Switch | Usar `@radix-ui/react-switch` com styling custom |
| ❌ Custom tooltip | Radix UI Tooltip | Usar `@radix-ui/react-tooltip` |
| ❌ Packet checksum from scratch | É um XOR/sum — trivial | OK, são 3 linhas de código, não vale a dep |
| ❌ EQ band interaction from scratch | Nenhuma lib boa para isto | OK, custom é o caminho correcto aqui |

### Sugestão: adicionar Radix UI Primitives

```
npm install @radix-ui/react-slider @radix-ui/react-switch @radix-ui/react-tooltip @radix-ui/react-select
```

Isto dá-nos acessibilidade (ARIA), keyboard navigation, e focus management de graça — só fazemos o styling. Não reinventamos a interacção.

---

## 5. Originalidade — Estamos a Criar, Não Apenas Copiar?

### Comparação com as fontes:

| Aspecto | OpenSCQ30 | baseus-desktop | Mabel |
|---------|-----------|----------------|-------|
| **Frontend** | GTK4 (Linux-first, ugly) | SolidJS, dark, premium | React, dark, Soundcore-branded |
| **Scope** | 20+ devices, massive | 1 device (earbuds), compact | 1 device (headphones), focused |
| **Architecture** | Macro-heavy, generic | Simple, direct | Simple, direct, typed |
| **Protocol** | Shared common parsers | Custom framing (0xAA) | Shared Soundcore framing (0x08/0x09) |
| **Transport** | RFCOMM (classic BT) | BLE GATT (btleplug) | RFCOMM (classic BT) |
| **Platform** | Linux + Windows + Android | Windows only | Windows (start), cross later |
| **UI concept** | Settings panel | Companion app | Premium desktop experience |

### O que é ORIGINAL na Mabel:

1. **Desktop-first premium UI** — nenhum dos projectos existentes tem uma UI bonita para desktop. OpenSCQ30 é funcional mas feio. Baseus é bonito mas mobile-sized.
2. **Single-device focus com depth** — em vez de suportar 20 devices mal, suportar 1 device muito bem.
3. **React ecosystem** — permite crescer para plugin marketplace, themes, etc.
4. **Simplified Rust backend** — sem macros, sem generics desnecessários. Direto e legível.
5. **Tauri v2** — mais moderno que qualquer dos projectos de referência.

### O que estamos a REUTILIZAR legitimamente (não copiar):

- **Protocolo** (byte layouts) — são factos de reverse engineering, não código
- **Design patterns** (trait, device loop, IPC bridge) — são standard da indústria
- **Approach** (RFCOMM via WinRT) — é a única forma de fazer no Windows

### Veredicto: ✅ Bom

Estamos a criar um produto original que usa conhecimento público (protocolo Soundcore documentado) e patterns comuns da indústria. O valor está na UI premium + experiência desktop + simplicidade do backend.

---

## Resumo de Acções

| # | Acção | Impacto |
|---|-------|---------|
| 1 | Adicionar Radix UI Primitives à Task A2 | Evita reinventar slider/toggle/select |
| 2 | Mover device loop para src-tauri (não para mabel-core) | Previne monolito no core |
| 3 | Simplificar Task A7 (EQ): v1 sem drag, só presets + static bars | Quick win mais rápido |
| 4 | **MANTER** Phase C5 (streaming parser) — é modular, ~40 LOC, protege contra fragmentação | Robustez sem custo |
| 5 | Reordenar Phase A: Home → Controls → Settings → ANC → EQ | Quick wins primeiro |
| 6 | Separar useDeviceState em 3 hooks menores | Evita God Object |
| 7 | Copiar MockTransport pattern do baseus-desktop para mabel-transport | Testes unitários day-1 |

---

## Conclusão

O plano está **sólido** no geral. As correcções acima são refinamentos, não reformulações. Os principais riscos eram:

- ~~Reinventar UI primitives~~ → Resolvido com Radix
- ~~Core monolítico~~ → Resolvido separando loop para o consumer
- ~~EQ complexa demais para quick win~~ → Resolvido simplificando v1 (sem drag)
- Streaming parser → **Mantido** (é modular, abstracto, ~40 LOC, zero risco de monolito)

**Recomendação:** aplicar estas correcções e avançar para Task A1.
