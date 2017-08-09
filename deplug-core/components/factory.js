import DissectorComponent from './dissector'
import FileComponent from './file'
import MenuComponent from './menu'
import PanelComponent from './panel'
import RendererComponent from './renderer'
import ScriptComponent from './script'
import StreamDissectorComponent from './stream-dissector'
import TabComponent from './tab'
import ThemeComponent from './theme'
import WindowComponent from './window'

export default class ComponentFactory {
  static create (rootDir, pkg, comp) {
    switch (comp.type) {
      case 'core:panel':
        return new PanelComponent(rootDir, pkg, comp)
      case 'core:tab':
        return new TabComponent(rootDir, pkg, comp)
      case 'core:theme':
        return new ThemeComponent(rootDir, pkg, comp)
      case 'core:window':
        return new WindowComponent(rootDir, pkg, comp)
      case 'core:dissector':
        return new DissectorComponent(rootDir, pkg, comp)
      case 'core:stream-dissector':
        return new StreamDissectorComponent(rootDir, pkg, comp)
      case 'core:menu':
        return new MenuComponent(rootDir, pkg, comp)
      case 'core:script':
        return new ScriptComponent(rootDir, pkg, comp)
      case 'core:file':
        return new FileComponent(rootDir, pkg, comp)
      case 'core:renderer':
        return new RendererComponent(rootDir, pkg, comp)
      default:
        throw new Error(`unknown component type: ${comp.type}`)
    }
  }
}
