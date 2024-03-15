var __TAURI_IIFE__=function(e){"use strict";function t(e,t,n,i){if("a"===n&&!i)throw new TypeError("Private accessor was defined without a getter");if("function"==typeof t?e!==t||!i:!t.has(e))throw new TypeError("Cannot read private member from an object whose class did not declare it");return"m"===n?i:"a"===n?i.call(e):i?i.value:t.get(e)}function n(e,t,n,i,r){if("m"===i)throw new TypeError("Private method is not writable");if("a"===i&&!r)throw new TypeError("Private accessor was defined without a setter");if("function"==typeof t?e!==t||!r:!t.has(e))throw new TypeError("Cannot write private member to an object whose class did not declare it");return"a"===i?r.call(e,n):r?r.value=n:t.set(e,n),n}var i,r,a,s;function l(e,t=!1){return window.__TAURI_INTERNALS__.transformCallback(e,t)}"function"==typeof SuppressedError&&SuppressedError;class o{constructor(){this.__TAURI_CHANNEL_MARKER__=!0,i.set(this,(()=>{})),r.set(this,0),a.set(this,{}),this.id=l((({message:e,id:s})=>{if(s===t(this,r,"f")){n(this,r,s+1,"f"),t(this,i,"f").call(this,e);const l=Object.keys(t(this,a,"f"));if(l.length>0){let e=s+1;for(const n of l.sort()){if(parseInt(n)!==e)break;{const r=t(this,a,"f")[n];delete t(this,a,"f")[n],t(this,i,"f").call(this,r),e+=1}}}}else t(this,a,"f")[s.toString()]=e}))}set onmessage(e){n(this,i,e,"f")}get onmessage(){return t(this,i,"f")}toJSON(){return`__CHANNEL__:${this.id}`}}i=new WeakMap,r=new WeakMap,a=new WeakMap;class u{constructor(e,t,n){this.plugin=e,this.event=t,this.channelId=n}async unregister(){return c(`plugin:${this.plugin}|remove_listener`,{event:this.event,channelId:this.channelId})}}async function c(e,t={},n){return window.__TAURI_INTERNALS__.invoke(e,t,n)}class d{get rid(){return t(this,s,"f")}constructor(e){s.set(this,void 0),n(this,s,e,"f")}async close(){return c("plugin:resources|close",{rid:this.rid})}}s=new WeakMap;var p=Object.freeze({__proto__:null,Channel:o,PluginListener:u,Resource:d,addPluginListener:async function(e,t,n){const i=new o;return i.onmessage=n,c(`plugin:${e}|register_listener`,{event:t,handler:i}).then((()=>new u(e,t,i.id)))},convertFileSrc:function(e,t="asset"){return window.__TAURI_INTERNALS__.convertFileSrc(e,t)},invoke:c,transformCallback:l});var h,y=Object.freeze({__proto__:null,getName:async function(){return c("plugin:app|name")},getTauriVersion:async function(){return c("plugin:app|tauri_version")},getVersion:async function(){return c("plugin:app|version")},hide:async function(){return c("plugin:app|app_hide")},show:async function(){return c("plugin:app|app_show")}});async function w(e,t){await c("plugin:event|unlisten",{event:e,eventId:t})}async function g(e,t,n){const i="string"==typeof n?.target?{kind:"AnyLabel",label:n.target}:n?.target??{kind:"Any"};return c("plugin:event|listen",{event:e,target:i,handler:l(t)}).then((t=>async()=>w(e,t)))}async function b(e,t,n){return g(e,(n=>{t(n),w(e,n.id).catch((()=>{}))}),n)}async function _(e,t){await c("plugin:event|emit",{event:e,payload:t})}async function m(e,t,n){const i="string"==typeof e?{kind:"AnyLabel",label:e}:e;await c("plugin:event|emit_to",{target:i,event:t,payload:n})}!function(e){e.WINDOW_RESIZED="tauri://resize",e.WINDOW_MOVED="tauri://move",e.WINDOW_CLOSE_REQUESTED="tauri://close-requested",e.WINDOW_DESTROYED="tauri://destroyed",e.WINDOW_FOCUS="tauri://focus",e.WINDOW_BLUR="tauri://blur",e.WINDOW_SCALE_FACTOR_CHANGED="tauri://scale-change",e.WINDOW_THEME_CHANGED="tauri://theme-changed",e.WEBVIEW_CREATED="tauri://webview-created",e.FILE_DROP="tauri://file-drop",e.FILE_DROP_HOVER="tauri://file-drop-hover",e.FILE_DROP_CANCELLED="tauri://file-drop-cancelled"}(h||(h={}));var f=Object.freeze({__proto__:null,get TauriEvent(){return h},emit:_,emitTo:m,listen:g,once:b});class v{constructor(e,t){this.type="Logical",this.width=e,this.height=t}}class k{constructor(e,t){this.type="Physical",this.width=e,this.height=t}toLogical(e){return new v(this.width/e,this.height/e)}}class A{constructor(e,t){this.type="Logical",this.x=e,this.y=t}}class E{constructor(e,t){this.type="Physical",this.x=e,this.y=t}toLogical(e){return new A(this.x/e,this.y/e)}}var D=Object.freeze({__proto__:null,LogicalPosition:A,LogicalSize:v,PhysicalPosition:E,PhysicalSize:k});class L extends d{constructor(e){super(e)}static async new(e,t,n){return c("plugin:image|new",{rgba:P(e),width:t,height:n}).then((e=>new L(e)))}static async fromBytes(e){return c("plugin:image|from_bytes",{bytes:P(e)}).then((e=>new L(e)))}static async fromPath(e){return c("plugin:image|from_path",{path:e}).then((e=>new L(e)))}async rgba(){return c("plugin:image|rgba",{rid:this.rid}).then((e=>new Uint8Array(e)))}async size(){return c("plugin:image|size",{rid:this.rid})}}function P(e){return null==e?null:"string"==typeof e?e:e instanceof Uint8Array?Array.from(e):e instanceof ArrayBuffer?Array.from(new Uint8Array(e)):e instanceof L?e.rid:e}var I,S,T=Object.freeze({__proto__:null,Image:L,transformImage:P});!function(e){e[e.Critical=1]="Critical",e[e.Informational=2]="Informational"}(I||(I={}));class C{constructor(e){this._preventDefault=!1,this.event=e.event,this.id=e.id}preventDefault(){this._preventDefault=!0}isPreventDefault(){return this._preventDefault}}function x(){return new O(window.__TAURI_INTERNALS__.metadata.currentWindow.label,{skip:!0})}function z(){return window.__TAURI_INTERNALS__.metadata.windows.map((e=>new O(e.label,{skip:!0})))}!function(e){e.None="none",e.Normal="normal",e.Indeterminate="indeterminate",e.Paused="paused",e.Error="error"}(S||(S={}));const R=["tauri://created","tauri://error"];class O{constructor(e,t={}){this.label=e,this.listeners=Object.create(null),t?.skip||c("plugin:window|create",{options:{...t,parent:"string"==typeof t.parent?t.parent:t.parent?.label,label:e}}).then((async()=>this.emit("tauri://created"))).catch((async e=>this.emit("tauri://error",e)))}static getByLabel(e){return z().find((t=>t.label===e))??null}static getCurrent(){return x()}static getAll(){return z()}static async getFocusedWindow(){for(const e of z())if(await e.isFocused())return e;return null}async listen(e,t){return this._handleTauriEvent(e,t)?Promise.resolve((()=>{const n=this.listeners[e];n.splice(n.indexOf(t),1)})):g(e,t,{target:{kind:"Window",label:this.label}})}async once(e,t){return this._handleTauriEvent(e,t)?Promise.resolve((()=>{const n=this.listeners[e];n.splice(n.indexOf(t),1)})):b(e,t,{target:{kind:"Window",label:this.label}})}async emit(e,t){if(R.includes(e)){for(const n of this.listeners[e]||[])n({event:e,id:-1,payload:t});return Promise.resolve()}return _(e,t)}async emitTo(e,t,n){if(R.includes(t)){for(const e of this.listeners[t]||[])e({event:t,id:-1,payload:n});return Promise.resolve()}return m(e,t,n)}_handleTauriEvent(e,t){return!!R.includes(e)&&(e in this.listeners?this.listeners[e].push(t):this.listeners[e]=[t],!0)}async scaleFactor(){return c("plugin:window|scale_factor",{label:this.label})}async innerPosition(){return c("plugin:window|inner_position",{label:this.label}).then((({x:e,y:t})=>new E(e,t)))}async outerPosition(){return c("plugin:window|outer_position",{label:this.label}).then((({x:e,y:t})=>new E(e,t)))}async innerSize(){return c("plugin:window|inner_size",{label:this.label}).then((({width:e,height:t})=>new k(e,t)))}async outerSize(){return c("plugin:window|outer_size",{label:this.label}).then((({width:e,height:t})=>new k(e,t)))}async isFullscreen(){return c("plugin:window|is_fullscreen",{label:this.label})}async isMinimized(){return c("plugin:window|is_minimized",{label:this.label})}async isMaximized(){return c("plugin:window|is_maximized",{label:this.label})}async isFocused(){return c("plugin:window|is_focused",{label:this.label})}async isDecorated(){return c("plugin:window|is_decorated",{label:this.label})}async isResizable(){return c("plugin:window|is_resizable",{label:this.label})}async isMaximizable(){return c("plugin:window|is_maximizable",{label:this.label})}async isMinimizable(){return c("plugin:window|is_minimizable",{label:this.label})}async isClosable(){return c("plugin:window|is_closable",{label:this.label})}async isVisible(){return c("plugin:window|is_visible",{label:this.label})}async title(){return c("plugin:window|title",{label:this.label})}async theme(){return c("plugin:window|theme",{label:this.label})}async center(){return c("plugin:window|center",{label:this.label})}async requestUserAttention(e){let t=null;return e&&(t=e===I.Critical?{type:"Critical"}:{type:"Informational"}),c("plugin:window|request_user_attention",{label:this.label,value:t})}async setResizable(e){return c("plugin:window|set_resizable",{label:this.label,value:e})}async setMaximizable(e){return c("plugin:window|set_maximizable",{label:this.label,value:e})}async setMinimizable(e){return c("plugin:window|set_minimizable",{label:this.label,value:e})}async setClosable(e){return c("plugin:window|set_closable",{label:this.label,value:e})}async setTitle(e){return c("plugin:window|set_title",{label:this.label,value:e})}async maximize(){return c("plugin:window|maximize",{label:this.label})}async unmaximize(){return c("plugin:window|unmaximize",{label:this.label})}async toggleMaximize(){return c("plugin:window|toggle_maximize",{label:this.label})}async minimize(){return c("plugin:window|minimize",{label:this.label})}async unminimize(){return c("plugin:window|unminimize",{label:this.label})}async show(){return c("plugin:window|show",{label:this.label})}async hide(){return c("plugin:window|hide",{label:this.label})}async close(){return c("plugin:window|close",{label:this.label})}async destroy(){return c("plugin:window|destroy",{label:this.label})}async setDecorations(e){return c("plugin:window|set_decorations",{label:this.label,value:e})}async setShadow(e){return c("plugin:window|set_shadow",{label:this.label,value:e})}async setEffects(e){return c("plugin:window|set_effects",{label:this.label,value:e})}async clearEffects(){return c("plugin:window|set_effects",{label:this.label,value:null})}async setAlwaysOnTop(e){return c("plugin:window|set_always_on_top",{label:this.label,value:e})}async setAlwaysOnBottom(e){return c("plugin:window|set_always_on_bottom",{label:this.label,value:e})}async setContentProtected(e){return c("plugin:window|set_content_protected",{label:this.label,value:e})}async setSize(e){if(!e||"Logical"!==e.type&&"Physical"!==e.type)throw new Error("the `size` argument must be either a LogicalSize or a PhysicalSize instance");return c("plugin:window|set_size",{label:this.label,value:{type:e.type,data:{width:e.width,height:e.height}}})}async setMinSize(e){if(e&&"Logical"!==e.type&&"Physical"!==e.type)throw new Error("the `size` argument must be either a LogicalSize or a PhysicalSize instance");return c("plugin:window|set_min_size",{label:this.label,value:e?{type:e.type,data:{width:e.width,height:e.height}}:null})}async setMaxSize(e){if(e&&"Logical"!==e.type&&"Physical"!==e.type)throw new Error("the `size` argument must be either a LogicalSize or a PhysicalSize instance");return c("plugin:window|set_max_size",{label:this.label,value:e?{type:e.type,data:{width:e.width,height:e.height}}:null})}async setPosition(e){if(!e||"Logical"!==e.type&&"Physical"!==e.type)throw new Error("the `position` argument must be either a LogicalPosition or a PhysicalPosition instance");return c("plugin:window|set_position",{label:this.label,value:{type:e.type,data:{x:e.x,y:e.y}}})}async setFullscreen(e){return c("plugin:window|set_fullscreen",{label:this.label,value:e})}async setFocus(){return c("plugin:window|set_focus",{label:this.label})}async setIcon(e){return c("plugin:window|set_icon",{label:this.label,value:P(e)})}async setSkipTaskbar(e){return c("plugin:window|set_skip_taskbar",{label:this.label,value:e})}async setCursorGrab(e){return c("plugin:window|set_cursor_grab",{label:this.label,value:e})}async setCursorVisible(e){return c("plugin:window|set_cursor_visible",{label:this.label,value:e})}async setCursorIcon(e){return c("plugin:window|set_cursor_icon",{label:this.label,value:e})}async setCursorPosition(e){if(!e||"Logical"!==e.type&&"Physical"!==e.type)throw new Error("the `position` argument must be either a LogicalPosition or a PhysicalPosition instance");return c("plugin:window|set_cursor_position",{label:this.label,value:{type:e.type,data:{x:e.x,y:e.y}}})}async setIgnoreCursorEvents(e){return c("plugin:window|set_ignore_cursor_events",{label:this.label,value:e})}async startDragging(){return c("plugin:window|start_dragging",{label:this.label})}async startResizeDragging(e){return c("plugin:window|start_resize_dragging",{label:this.label,value:e})}async setProgressBar(e){return c("plugin:window|set_progress_bar",{label:this.label,value:e})}async setVisibleOnAllWorkspaces(e){return c("plugin:window|set_visible_on_all_workspaces",{label:this.label,value:e})}async onResized(e){return this.listen(h.WINDOW_RESIZED,(t=>{t.payload=U(t.payload),e(t)}))}async onMoved(e){return this.listen(h.WINDOW_MOVED,(t=>{t.payload=M(t.payload),e(t)}))}async onCloseRequested(e){return this.listen(h.WINDOW_CLOSE_REQUESTED,(t=>{const n=new C(t);Promise.resolve(e(n)).then((()=>{if(!n.isPreventDefault())return this.destroy()}))}))}async onFileDropEvent(e){const t=await this.listen(h.FILE_DROP,(t=>{e({...t,payload:{type:"drop",paths:t.payload.paths,position:M(t.payload.position)}})})),n=await this.listen(h.FILE_DROP_HOVER,(t=>{e({...t,payload:{type:"hover",paths:t.payload.paths,position:M(t.payload.position)}})})),i=await this.listen(h.FILE_DROP_CANCELLED,(t=>{e({...t,payload:{type:"cancel"}})}));return()=>{t(),n(),i()}}async onFocusChanged(e){const t=await this.listen(h.WINDOW_FOCUS,(t=>{e({...t,payload:!0})})),n=await this.listen(h.WINDOW_BLUR,(t=>{e({...t,payload:!1})}));return()=>{t(),n()}}async onScaleChanged(e){return this.listen(h.WINDOW_SCALE_FACTOR_CHANGED,e)}async onThemeChanged(e){return this.listen(h.WINDOW_THEME_CHANGED,e)}}var F,W;function N(e){return null===e?null:{name:e.name,scaleFactor:e.scaleFactor,position:M(e.position),size:U(e.size)}}function M(e){return new E(e.x,e.y)}function U(e){return new k(e.width,e.height)}!function(e){e.AppearanceBased="appearanceBased",e.Light="light",e.Dark="dark",e.MediumLight="mediumLight",e.UltraDark="ultraDark",e.Titlebar="titlebar",e.Selection="selection",e.Menu="menu",e.Popover="popover",e.Sidebar="sidebar",e.HeaderView="headerView",e.Sheet="sheet",e.WindowBackground="windowBackground",e.HudWindow="hudWindow",e.FullScreenUI="fullScreenUI",e.Tooltip="tooltip",e.ContentBackground="contentBackground",e.UnderWindowBackground="underWindowBackground",e.UnderPageBackground="underPageBackground",e.Mica="mica",e.Blur="blur",e.Acrylic="acrylic",e.Tabbed="tabbed",e.TabbedDark="tabbedDark",e.TabbedLight="tabbedLight"}(F||(F={})),function(e){e.FollowsWindowActiveState="followsWindowActiveState",e.Active="active",e.Inactive="inactive"}(W||(W={}));var B=Object.freeze({__proto__:null,CloseRequestedEvent:C,get Effect(){return F},get EffectState(){return W},LogicalPosition:A,LogicalSize:v,PhysicalPosition:E,PhysicalSize:k,get ProgressBarStatus(){return S},get UserAttentionType(){return I},Window:O,availableMonitors:async function(){return c("plugin:window|available_monitors").then((e=>e.map(N)))},currentMonitor:async function(){return c("plugin:window|current_monitor").then(N)},getAll:z,getCurrent:x,primaryMonitor:async function(){return c("plugin:window|primary_monitor").then(N)}});function j(){return new G(x(),window.__TAURI_INTERNALS__.metadata.currentWebview.label,{skip:!0})}function V(){return window.__TAURI_INTERNALS__.metadata.webviews.map((e=>new G(O.getByLabel(e.windowLabel),e.label,{skip:!0})))}const H=["tauri://created","tauri://error"];class G{constructor(e,t,n){this.window=e,this.label=t,this.listeners=Object.create(null),n?.skip||c("plugin:webview|create_webview",{windowLabel:e.label,label:t,options:n}).then((async()=>this.emit("tauri://created"))).catch((async e=>this.emit("tauri://error",e)))}static getByLabel(e){return V().find((t=>t.label===e))??null}static getCurrent(){return j()}static getAll(){return V()}async listen(e,t){return this._handleTauriEvent(e,t)?Promise.resolve((()=>{const n=this.listeners[e];n.splice(n.indexOf(t),1)})):g(e,t,{target:{kind:"Webview",label:this.label}})}async once(e,t){return this._handleTauriEvent(e,t)?Promise.resolve((()=>{const n=this.listeners[e];n.splice(n.indexOf(t),1)})):b(e,t,{target:{kind:"Webview",label:this.label}})}async emit(e,t){if(H.includes(e)){for(const n of this.listeners[e]||[])n({event:e,id:-1,payload:t});return Promise.resolve()}return _(e,t)}async emitTo(e,t,n){if(H.includes(t)){for(const e of this.listeners[t]||[])e({event:t,id:-1,payload:n});return Promise.resolve()}return m(e,t,n)}_handleTauriEvent(e,t){return!!H.includes(e)&&(e in this.listeners?this.listeners[e].push(t):this.listeners[e]=[t],!0)}async position(){return c("plugin:webview|webview_position",{label:this.label}).then((({x:e,y:t})=>new E(e,t)))}async size(){return c("plugin:webview|webview_size",{label:this.label}).then((({width:e,height:t})=>new k(e,t)))}async close(){return c("plugin:webview|close",{label:this.label})}async setSize(e){if(!e||"Logical"!==e.type&&"Physical"!==e.type)throw new Error("the `size` argument must be either a LogicalSize or a PhysicalSize instance");return c("plugin:webview|set_webview_size",{label:this.label,value:{type:e.type,data:{width:e.width,height:e.height}}})}async setPosition(e){if(!e||"Logical"!==e.type&&"Physical"!==e.type)throw new Error("the `position` argument must be either a LogicalPosition or a PhysicalPosition instance");return c("plugin:webview|set_webview_position",{label:this.label,value:{type:e.type,data:{x:e.x,y:e.y}}})}async setFocus(){return c("plugin:webview|set_webview_focus",{label:this.label})}async reparent(e){return c("plugin:webview|set_webview_focus",{label:this.label,window:"string"==typeof e?e:e.label})}async onFileDropEvent(e){const t=await this.listen(h.FILE_DROP,(t=>{e({...t,payload:{type:"drop",paths:t.payload.paths,position:q(t.payload.position)}})})),n=await this.listen(h.FILE_DROP_HOVER,(t=>{e({...t,payload:{type:"hover",paths:t.payload.paths,position:q(t.payload.position)}})})),i=await this.listen(h.FILE_DROP_CANCELLED,(t=>{e({...t,payload:{type:"cancel"}})}));return()=>{t(),n(),i()}}}function q(e){return new E(e.x,e.y)}var Q,$,Z=Object.freeze({__proto__:null,Webview:G,getAll:V,getCurrent:j});function J(){const e=j();return new Y(e.label,{skip:!0})}function K(){return window.__TAURI_INTERNALS__.metadata.webviews.map((e=>new Y(e.label,{skip:!0})))}class Y{constructor(e,t={}){this.label=e,this.listeners=Object.create(null),t?.skip||c("plugin:webview|create_webview_window",{options:{...t,parent:"string"==typeof t.parent?t.parent:t.parent?.label,label:e}}).then((async()=>this.emit("tauri://created"))).catch((async e=>this.emit("tauri://error",e)))}static getByLabel(e){const t=K().find((t=>t.label===e))??null;return t?new Y(t.label,{skip:!0}):null}static getCurrent(){return J()}static getAll(){return K().map((e=>new Y(e.label,{skip:!0})))}async listen(e,t){return this._handleTauriEvent(e,t)?Promise.resolve((()=>{const n=this.listeners[e];n.splice(n.indexOf(t),1)})):g(e,t,{target:{kind:"WebviewWindow",label:this.label}})}async once(e,t){return this._handleTauriEvent(e,t)?Promise.resolve((()=>{const n=this.listeners[e];n.splice(n.indexOf(t),1)})):b(e,t,{target:{kind:"WebviewWindow",label:this.label}})}}Q=Y,$=[O,G],(Array.isArray($)?$:[$]).forEach((e=>{Object.getOwnPropertyNames(e.prototype).forEach((t=>{"object"==typeof Q.prototype&&Q.prototype&&t in Q.prototype||Object.defineProperty(Q.prototype,t,Object.getOwnPropertyDescriptor(e.prototype,t)??Object.create(null))}))}));var X,ee=Object.freeze({__proto__:null,WebviewWindow:Y,getAll:K,getCurrent:J});!function(e){e[e.Audio=1]="Audio",e[e.Cache=2]="Cache",e[e.Config=3]="Config",e[e.Data=4]="Data",e[e.LocalData=5]="LocalData",e[e.Document=6]="Document",e[e.Download=7]="Download",e[e.Picture=8]="Picture",e[e.Public=9]="Public",e[e.Video=10]="Video",e[e.Resource=11]="Resource",e[e.Temp=12]="Temp",e[e.AppConfig=13]="AppConfig",e[e.AppData=14]="AppData",e[e.AppLocalData=15]="AppLocalData",e[e.AppCache=16]="AppCache",e[e.AppLog=17]="AppLog",e[e.Desktop=18]="Desktop",e[e.Executable=19]="Executable",e[e.Font=20]="Font",e[e.Home=21]="Home",e[e.Runtime=22]="Runtime",e[e.Template=23]="Template"}(X||(X={}));var te=Object.freeze({__proto__:null,get BaseDirectory(){return X},appCacheDir:async function(){return c("plugin:path|resolve_directory",{directory:X.AppCache})},appConfigDir:async function(){return c("plugin:path|resolve_directory",{directory:X.AppConfig})},appDataDir:async function(){return c("plugin:path|resolve_directory",{directory:X.AppData})},appLocalDataDir:async function(){return c("plugin:path|resolve_directory",{directory:X.AppLocalData})},appLogDir:async function(){return c("plugin:path|resolve_directory",{directory:X.AppLog})},audioDir:async function(){return c("plugin:path|resolve_directory",{directory:X.Audio})},basename:async function(e,t){return c("plugin:path|basename",{path:e,ext:t})},cacheDir:async function(){return c("plugin:path|resolve_directory",{directory:X.Cache})},configDir:async function(){return c("plugin:path|resolve_directory",{directory:X.Config})},dataDir:async function(){return c("plugin:path|resolve_directory",{directory:X.Data})},delimiter:function(){return window.__TAURI_INTERNALS__.plugins.path.delimiter},desktopDir:async function(){return c("plugin:path|resolve_directory",{directory:X.Desktop})},dirname:async function(e){return c("plugin:path|dirname",{path:e})},documentDir:async function(){return c("plugin:path|resolve_directory",{directory:X.Document})},downloadDir:async function(){return c("plugin:path|resolve_directory",{directory:X.Download})},executableDir:async function(){return c("plugin:path|resolve_directory",{directory:X.Executable})},extname:async function(e){return c("plugin:path|extname",{path:e})},fontDir:async function(){return c("plugin:path|resolve_directory",{directory:X.Font})},homeDir:async function(){return c("plugin:path|resolve_directory",{directory:X.Home})},isAbsolute:async function(e){return c("plugin:path|isAbsolute",{path:e})},join:async function(...e){return c("plugin:path|join",{paths:e})},localDataDir:async function(){return c("plugin:path|resolve_directory",{directory:X.LocalData})},normalize:async function(e){return c("plugin:path|normalize",{path:e})},pictureDir:async function(){return c("plugin:path|resolve_directory",{directory:X.Picture})},publicDir:async function(){return c("plugin:path|resolve_directory",{directory:X.Public})},resolve:async function(...e){return c("plugin:path|resolve",{paths:e})},resolveResource:async function(e){return c("plugin:path|resolve_directory",{directory:X.Resource,path:e})},resourceDir:async function(){return c("plugin:path|resolve_directory",{directory:X.Resource})},runtimeDir:async function(){return c("plugin:path|resolve_directory",{directory:X.Runtime})},sep:function(){return window.__TAURI_INTERNALS__.plugins.path.sep},tempDir:async function(){return c("plugin:path|resolve_directory",{directory:X.Temp})},templateDir:async function(){return c("plugin:path|resolve_directory",{directory:X.Template})},videoDir:async function(){return c("plugin:path|resolve_directory",{directory:X.Video})}});class ne extends d{constructor(e,t){super(e),this.id=t}static async getById(e){return c("plugin:tray|get_by_id",{id:e}).then((t=>t?new ne(t,e):null))}static async removeById(e){return c("plugin:tray|remove_by_id",{id:e})}static async new(e){e?.menu&&(e.menu=[e.menu.rid,e.menu.kind]),e?.icon&&(e.icon=P(e.icon));const t=new o;return e?.action&&(t.onmessage=e.action,delete e.action),c("plugin:tray|new",{options:e??{},handler:t}).then((([e,t])=>new ne(e,t)))}async setIcon(e){let t=null;return e&&(t=P(e)),c("plugin:tray|set_icon",{rid:this.rid,icon:t})}async setMenu(e){return e&&(e=[e.rid,e.kind]),c("plugin:tray|set_menu",{rid:this.rid,menu:e})}async setTooltip(e){return c("plugin:tray|set_tooltip",{rid:this.rid,tooltip:e})}async setTitle(e){return c("plugin:tray|set_title",{rid:this.rid,title:e})}async setVisible(e){return c("plugin:tray|set_visible",{rid:this.rid,visible:e})}async setTempDirPath(e){return c("plugin:tray|set_temp_dir_path",{rid:this.rid,path:e})}async setIconAsTemplate(e){return c("plugin:tray|set_icon_as_template",{rid:this.rid,asTemplate:e})}async setMenuOnLeftClick(e){return c("plugin:tray|set_show_menu_on_left_click",{rid:this.rid,onLeft:e})}}var ie,re,ae,se=Object.freeze({__proto__:null,TrayIcon:ne});function le(e){if("items"in e)e.items=e.items?.map((e=>"rid"in e?e:le(e)));else if("action"in e&&e.action){const t=new o;return t.onmessage=e.action,delete e.action,{...e,handler:t}}return e}async function oe(e,t){const n=new o;let i=null;return t&&"object"==typeof t&&("action"in t&&t.action&&(n.onmessage=t.action,delete t.action),"items"in t&&t.items&&(i=t.items.map((e=>"rid"in e?[e.rid,e.kind]:("item"in e&&"object"==typeof e.item&&e.item.About?.icon&&(e.item.About.icon=P(e.item.About.icon)),"icon"in e&&e.icon&&(e.icon=P(e.icon)),le(e)))))),c("plugin:menu|new",{kind:e,options:t?{...t,items:i}:void 0,handler:n})}class ue extends d{get id(){return t(this,ie,"f")}get kind(){return t(this,re,"f")}constructor(e,t,i){super(e),ie.set(this,void 0),re.set(this,void 0),n(this,ie,t,"f"),n(this,re,i,"f")}}ie=new WeakMap,re=new WeakMap;class ce extends ue{constructor(e,t){super(e,t,"MenuItem")}static async new(e){return oe("MenuItem",e).then((([e,t])=>new ce(e,t)))}async text(){return c("plugin:menu|text",{rid:this.rid,kind:this.kind})}async setText(e){return c("plugin:menu|set_text",{rid:this.rid,kind:this.kind,text:e})}async isEnabled(){return c("plugin:menu|is_enabled",{rid:this.rid,kind:this.kind})}async setEnabled(e){return c("plugin:menu|set_enabled",{rid:this.rid,kind:this.kind,enabled:e})}async setAccelerator(e){return c("plugin:menu|set_accelerator",{rid:this.rid,kind:this.kind,accelerator:e})}}class de extends ue{constructor(e,t){super(e,t,"Check")}static async new(e){return oe("Check",e).then((([e,t])=>new de(e,t)))}async text(){return c("plugin:menu|text",{rid:this.rid,kind:this.kind})}async setText(e){return c("plugin:menu|set_text",{rid:this.rid,kind:this.kind,text:e})}async isEnabled(){return c("plugin:menu|is_enabled",{rid:this.rid,kind:this.kind})}async setEnabled(e){return c("plugin:menu|set_enabled",{rid:this.rid,kind:this.kind,enabled:e})}async setAccelerator(e){return c("plugin:menu|set_accelerator",{rid:this.rid,kind:this.kind,accelerator:e})}async isChecked(){return c("plugin:menu|is_checked",{rid:this.rid})}async setChecked(e){return c("plugin:menu|set_checked",{rid:this.rid,checked:e})}}!function(e){e.Add="Add",e.Advanced="Advanced",e.Bluetooth="Bluetooth",e.Bookmarks="Bookmarks",e.Caution="Caution",e.ColorPanel="ColorPanel",e.ColumnView="ColumnView",e.Computer="Computer",e.EnterFullScreen="EnterFullScreen",e.Everyone="Everyone",e.ExitFullScreen="ExitFullScreen",e.FlowView="FlowView",e.Folder="Folder",e.FolderBurnable="FolderBurnable",e.FolderSmart="FolderSmart",e.FollowLinkFreestanding="FollowLinkFreestanding",e.FontPanel="FontPanel",e.GoLeft="GoLeft",e.GoRight="GoRight",e.Home="Home",e.IChatTheater="IChatTheater",e.IconView="IconView",e.Info="Info",e.InvalidDataFreestanding="InvalidDataFreestanding",e.LeftFacingTriangle="LeftFacingTriangle",e.ListView="ListView",e.LockLocked="LockLocked",e.LockUnlocked="LockUnlocked",e.MenuMixedState="MenuMixedState",e.MenuOnState="MenuOnState",e.MobileMe="MobileMe",e.MultipleDocuments="MultipleDocuments",e.Network="Network",e.Path="Path",e.PreferencesGeneral="PreferencesGeneral",e.QuickLook="QuickLook",e.RefreshFreestanding="RefreshFreestanding",e.Refresh="Refresh",e.Remove="Remove",e.RevealFreestanding="RevealFreestanding",e.RightFacingTriangle="RightFacingTriangle",e.Share="Share",e.Slideshow="Slideshow",e.SmartBadge="SmartBadge",e.StatusAvailable="StatusAvailable",e.StatusNone="StatusNone",e.StatusPartiallyAvailable="StatusPartiallyAvailable",e.StatusUnavailable="StatusUnavailable",e.StopProgressFreestanding="StopProgressFreestanding",e.StopProgress="StopProgress",e.TrashEmpty="TrashEmpty",e.TrashFull="TrashFull",e.User="User",e.UserAccounts="UserAccounts",e.UserGroup="UserGroup",e.UserGuest="UserGuest"}(ae||(ae={}));class pe extends ue{constructor(e,t){super(e,t,"Icon")}static async new(e){return oe("Icon",e).then((([e,t])=>new pe(e,t)))}async text(){return c("plugin:menu|text",{rid:this.rid,kind:this.kind})}async setText(e){return c("plugin:menu|set_text",{rid:this.rid,kind:this.kind,text:e})}async isEnabled(){return c("plugin:menu|is_enabled",{rid:this.rid,kind:this.kind})}async setEnabled(e){return c("plugin:menu|set_enabled",{rid:this.rid,kind:this.kind,enabled:e})}async setAccelerator(e){return c("plugin:menu|set_accelerator",{rid:this.rid,kind:this.kind,accelerator:e})}async setIcon(e){return c("plugin:menu|set_icon",{rid:this.rid,icon:P(e)})}}class he extends ue{constructor(e,t){super(e,t,"Predefined")}static async new(e){return oe("Predefined",e).then((([e,t])=>new he(e,t)))}async text(){return c("plugin:menu|text",{rid:this.rid,kind:this.kind})}async setText(e){return c("plugin:menu|set_text",{rid:this.rid,kind:this.kind,text:e})}}function ye([e,t,n]){switch(n){case"Submenu":return new we(e,t);case"Predefined":return new he(e,t);case"Check":return new de(e,t);case"Icon":return new pe(e,t);default:return new ce(e,t)}}class we extends ue{constructor(e,t){super(e,t,"Submenu")}static async new(e){return oe("Submenu",e).then((([e,t])=>new we(e,t)))}async text(){return c("plugin:menu|text",{rid:this.rid,kind:this.kind})}async setText(e){return c("plugin:menu|set_text",{rid:this.rid,kind:this.kind,text:e})}async isEnabled(){return c("plugin:menu|is_enabled",{rid:this.rid,kind:this.kind})}async setEnabled(e){return c("plugin:menu|set_enabled",{rid:this.rid,kind:this.kind,enabled:e})}async append(e){return c("plugin:menu|append",{rid:this.rid,kind:this.kind,items:(Array.isArray(e)?e:[e]).map((e=>"rid"in e?[e.rid,e.kind]:e))})}async prepend(e){return c("plugin:menu|prepend",{rid:this.rid,kind:this.kind,items:(Array.isArray(e)?e:[e]).map((e=>"rid"in e?[e.rid,e.kind]:e))})}async insert(e,t){return c("plugin:menu|insert",{rid:this.rid,kind:this.kind,items:(Array.isArray(e)?e:[e]).map((e=>"rid"in e?[e.rid,e.kind]:e)),position:t})}async remove(e){return c("plugin:menu|remove",{rid:this.rid,kind:this.kind,item:[e.rid,e.kind]})}async removeAt(e){return c("plugin:menu|remove_at",{rid:this.rid,kind:this.kind,position:e}).then(ye)}async items(){return c("plugin:menu|items",{rid:this.rid,kind:this.kind}).then((e=>e.map(ye)))}async get(e){return c("plugin:menu|get",{rid:this.rid,kind:this.kind,id:e}).then((e=>e?ye(e):null))}async popup(e,t){let n=null;return e&&(n={type:e instanceof E?"Physical":"Logical",data:e}),c("plugin:menu|popup",{rid:this.rid,kind:this.kind,window:t?.label??null,at:n})}async setAsWindowsMenuForNSApp(){return c("plugin:menu|set_as_windows_menu_for_nsapp",{rid:this.rid})}async setAsHelpMenuForNSApp(){return c("plugin:menu|set_as_help_menu_for_nsapp",{rid:this.rid})}}function ge([e,t,n]){switch(n){case"Submenu":return new we(e,t);case"Predefined":return new he(e,t);case"Check":return new de(e,t);case"Icon":return new pe(e,t);default:return new ce(e,t)}}class be extends ue{constructor(e,t){super(e,t,"Menu")}static async new(e){return oe("Menu",e).then((([e,t])=>new be(e,t)))}static async default(){return c("plugin:menu|create_default").then((([e,t])=>new be(e,t)))}async append(e){return c("plugin:menu|append",{rid:this.rid,kind:this.kind,items:(Array.isArray(e)?e:[e]).map((e=>"rid"in e?[e.rid,e.kind]:e))})}async prepend(e){return c("plugin:menu|prepend",{rid:this.rid,kind:this.kind,items:(Array.isArray(e)?e:[e]).map((e=>"rid"in e?[e.rid,e.kind]:e))})}async insert(e,t){return c("plugin:menu|insert",{rid:this.rid,kind:this.kind,items:(Array.isArray(e)?e:[e]).map((e=>"rid"in e?[e.rid,e.kind]:e)),position:t})}async remove(e){return c("plugin:menu|remove",{rid:this.rid,kind:this.kind,item:[e.rid,e.kind]})}async removeAt(e){return c("plugin:menu|remove_at",{rid:this.rid,kind:this.kind,position:e}).then(ge)}async items(){return c("plugin:menu|items",{rid:this.rid,kind:this.kind}).then((e=>e.map(ge)))}async get(e){return c("plugin:menu|get",{rid:this.rid,kind:this.kind,id:e}).then((e=>e?ge(e):null))}async popup(e,t){let n=null;return e&&(n={type:e instanceof E?"Physical":"Logical",data:e}),c("plugin:menu|popup",{rid:this.rid,kind:this.kind,window:t?.label??null,at:n})}async setAsAppMenu(){return c("plugin:menu|set_as_app_menu",{rid:this.rid}).then((e=>e?new be(e[0],e[1]):null))}async setAsWindowMenu(e){return c("plugin:menu|set_as_window_menu",{rid:this.rid,window:e?.label??null}).then((e=>e?new be(e[0],e[1]):null))}}var _e=Object.freeze({__proto__:null,CheckMenuItem:de,IconMenuItem:pe,Menu:be,MenuItem:ce,get NativeIcon(){return ae},PredefinedMenuItem:he,Submenu:we});return e.app=y,e.core=p,e.dpi=D,e.event=f,e.image=T,e.menu=_e,e.path=te,e.tray=se,e.webview=Z,e.webviewWindow=ee,e.window=B,e}({});window.__TAURI__=__TAURI_IIFE__;
