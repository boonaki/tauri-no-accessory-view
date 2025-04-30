import SwiftRs
import Tauri
import UIKit
import WebKit

class PingArgs: Decodable {
  let value: String?
}

//class RichEditorWebView: WKWebView {
//
//    var accessoryView: UIView? = nil
//
//    override var inputAccessoryView: UIView? {
//        // remove/replace the default accessory view
//        return accessoryView
//    }
//
//}
// final class RichEditorWebView: NSObject {
//     @objc var inputAccessoryView: AnyObject? { return nil }
// }

class ExamplePlugin: Plugin {
    
    @objc public func ping(_ invoke: Invoke) throws {
      let args = try invoke.parseArgs(PingArgs.self)
      invoke.resolve(["value": args.value ?? ""])
    }
    
    @objc public override func load(webview: WKWebView) {
      guard let targetClass = NSClassFromString("WKContentView") else { return }

      let selector = sel_registerName("inputAccessoryView")
      
      let block: @convention(block) (AnyObject) -> UIView? = { _ in return nil }
      let imp = imp_implementationWithBlock(block)

      if let method = class_getInstanceMethod(targetClass, selector) {
        class_replaceMethod(targetClass, selector, imp, method_getTypeEncoding(method))
      }
    }
}

@_cdecl("init_plugin_no_accessory_view")
func initPlugin() -> Plugin {
  return ExamplePlugin()
}
