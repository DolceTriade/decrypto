(this["webpackJsonpdecrypto-frontend"]=this["webpackJsonpdecrypto-frontend"]||[]).push([[0],{107:function(e,t,a){e.exports=a(127)},127:function(e,t,a){"use strict";a.r(t);var n=a(0),r=a.n(n),s=a(9),l=a.n(s),i=a(67),c=a(29),o=a(45),m=a(46),u=a(55),h=a(47),d=a(56),p=a(130),g=a(4),E=a(179),b=a(57),y=a(178),f=a(190),v=a(33),_=a(171),k=a(170),S=a(172),w=a(173),C=a(174),j=a(177),x=a(176),N=a(175),O=function(e){function t(e){var a;return Object(o.a)(this,t),(a=Object(u.a)(this,Object(h.a)(t).call(this,e))).state={s:null,connected:!1},a}return Object(d.a)(t,e),Object(m.a)(t,[{key:"createSocket",value:function(e){var t=this,a="ws://"+document.domain+":"+window.location.port+"/lobby_ws";var n=new WebSocket(a),r=this;return n.onmessage=function(t){console.log("Got server response:"),console.log(t);var a=null;try{a=JSON.parse(t.data)}catch(s){return void console.log(s)}if("command"in a)switch(a.command){case"join_game":var n=new URL(a.game);e.history.push({pathname:n.pathname});break;case"error":console.log("ERROR: "+a.msg),r.props.enqueueSnackbar(a.msg,{variant:"error"});break;default:console.log("Unhandled command: "+a)}else console.log("No command in response from server. "+t)},n.onopen=function(e){return t.setState({connected:!0})},n.onclose=function(e){return t.setState({connected:!1})},n}},{key:"join_or_create",value:function(){var e=document.getElementById("name").value,t=document.getElementById("room").value;e?t?this.state.s.send(JSON.stringify({command:"join_or_create_game",name:e,room:t})):console.log("ERROR: Room cannot be empty."):console.log("ERROR: Name cannot be empty.")}},{key:"componentDidMount",value:function(){console.log("LOBBY MOUNT"),console.log(this.state),null===this.state.s&&this.setState({s:this.createSocket(this.props)})}},{key:"componentWillUnmount",value:function(){console.log("LOBBY UNMOUNT"),null===this.state.s&&(this.state.s.close(),this.setState({s:null}))}},{key:"render",value:function(){var e=this.props.classes;return r.a.createElement(r.a.Fragment,null,r.a.createElement(k.a,null),r.a.createElement(_.a,{position:"relative"},r.a.createElement(S.a,null,r.a.createElement(p.a,{variant:"h6",color:"inherit",noWrap:!0},"Decrypto"))),r.a.createElement(w.a,{open:!this.state.connected},r.a.createElement(C.a,null,r.a.createElement(N.a,{container:!0,justify:"center"},r.a.createElement(x.a,{justify:"center",disableShrink:!0})),r.a.createElement(p.a,{component:"h2",align:"center"},"Waiting for WebSocket connection...")),r.a.createElement(j.a,null,r.a.createElement(y.a,{onClick:function(e){return window.location.reload(!1)},color:"primary"},"Refresh"))),r.a.createElement("div",{className:e.heroContent},r.a.createElement(E.a,{maxWidth:"sm"},r.a.createElement(p.a,{component:"h1",variant:"h2",align:"center",color:"textPrimary",gutterBottom:!0},"Decrypto"),r.a.createElement(p.a,{variant:"h5",align:"center",color:"textSecondary",paragraph:!0},"Enter a game name and user name to join or create a game!")),r.a.createElement(E.a,null,r.a.createElement(b.a,{align:"center"},r.a.createElement(f.a,{id:"name",label:"Name",className:e.textField,margin:"normal"}),r.a.createElement(f.a,{id:"room",label:"Room",className:e.textField,margin:"normal"}),r.a.createElement("br",null),r.a.createElement(y.a,{variant:"contained",color:"primary",className:e.heroButtons,onClick:this.join_or_create.bind(this)},"Join or Create Game")))))}}]),t}(r.a.Component),B=Object(v.withSnackbar)(Object(c.e)(Object(g.a)((function(e){return{icon:{marginRight:e.spacing(2)},heroContent:{backgroundColor:e.palette.background.paper,padding:e.spacing(8,0,6)},paper:{display:"flex",flexDirection:"column",marginBottom:e.spacing(1)},heroButtons:{marginTop:e.spacing(4),marginBottom:e.spacing(4)},textField:{marginLeft:e.spacing(1),marginRight:e.spacing(1),width:200}}}))(O))),R=a(37),T=a(70),G=a(43),W=a(187),D=a(186),M=a(188),F=a(86),I=a.n(F),P=a(85),U=a.n(P),J=a(192),L=a(180),A=a(193),Y=a(84),H=a.n(Y),z=a(191),q=a(189),V=a(183),K=a(185),Q=a(181),X=a(184),Z=a(182),$=function(e){return{icon:{marginRight:e.spacing(2)},heroContent:{width:"available",backgroundColor:e.palette.background.paper,padding:e.spacing(8,0,6)},paper:{display:"flex",flexDirection:"column",marginBottom:e.spacing(1)},heroButtons:{marginTop:e.spacing(4),marginBottom:e.spacing(4)},textField:{marginLeft:e.spacing(1),marginRight:e.spacing(1),width:200},padding:{padding:e.spacing(0,2),marginBottom:e.spacing(1),fontSize:12},teamName:{paddingBottom:e.spacing(1)},words:{padding:"0.25em"},clueField:{marginLeft:e.spacing(1),marginRight:e.spacing(1),width:200},card:{padding:e.spacing(1)},bottomBar:{position:"static",overflow:"none",height:e.spacing(15),backgroundColor:e.palette.background.paper},chatBox:{width:"100%",overflow:"auto",height:e.spacing(15),backgroundColor:e.palette.background.paper},appBar:{zIndex:2,elevation:0}}},ee=function(e){return Math.random().toString(36)},te=function(e){return{system:!0,name:"",message:e}};function ae(e){return r.a.createElement(N.a,{container:!0,spacing:2,justify:"center"},r.a.createElement(N.a,{item:!0},r.a.createElement(y.a,{variant:"contained",color:"primary",onClick:e.team_a},"Team A")),r.a.createElement(N.a,{item:!0},r.a.createElement(y.a,{variant:"contained",color:"primary",onClick:e.team_b},"Team B")))}function ne(e){return r.a.createElement(N.a,{container:!0,spacing:2,justify:"center"},r.a.createElement(N.a,{item:!0},r.a.createElement(y.a,{variant:"contained",color:"primary",onClick:e.onClick},"Leave Team")))}function re(e){return r.a.createElement(N.a,{container:!0,spacing:2,justify:"center"},r.a.createElement(N.a,{item:!0},r.a.createElement(y.a,{variant:"contained",color:"primary",onClick:e.onClick},"Start")))}function se(e){var t=[];return e.order.forEach((function(e){t.push(r.a.createElement(N.a,{item:!0,key:e},r.a.createElement(b.a,{p:3},r.a.createElement(p.a,{variant:"h5",align:"center"},e))))})),r.a.createElement(N.a,{container:!0,spacing:1,justify:"center"},t)}function le(e){var t=Object(G.a)($),a=null,n=[],s=[];if(e.round.clue_giver!==e.me||"guesses"in e.round)if("clues"in e.round&&!("guesses"in e.round)){for(c=0;c<3;++c){var l="guesses"+c;n.push(r.a.createElement(N.a,{item:!0,key:l},r.a.createElement(f.a,{id:l,name:c.toString(),autoComplete:"off",type:"number",label:e.round.clues[c],className:t.clueField,margin:"normal",onChange:e.setGuesses})))}s.push(r.a.createElement(y.a,{key:"submitg",variant:"contained",color:"primary",onClick:e.submitGuesses},"Submit Guesses"))}else if("spy_clues"in e.round&&!("team_spy_guesses"in e.round)&&"clues"in e.round){for(c=0;c<3;++c){var i="spyguesses"+c;n.push(r.a.createElement(N.a,{item:!0,key:i},r.a.createElement(f.a,{id:i,name:c.toString(),autoComplete:"off",type:"number",label:e.round.spy_clues[c],className:t.clueField,margin:"normal",onChange:e.setSpyGuesses})))}s.push(r.a.createElement(y.a,{key:"submitsg",variant:"contained",color:"primary",onClick:e.submitSpyGuesses},"Submit Spy Guesses"))}else"spy_order"in e.round&&"order"in e.round?(a=r.a.createElement(E.a,null,r.a.createElement(p.a,null,"Your order:"),r.a.createElement(se,{order:e.round.order}),r.a.createElement(p.a,null,"Your Guess:"),r.a.createElement(se,{order:e.round.guesses}),r.a.createElement(p.a,null,"Spies Order:"),r.a.createElement(se,{order:e.round.spy_order}),r.a.createElement(p.a,null,"Your Spy Guess:"),r.a.createElement(se,{order:e.round.team_spy_guesses})),n.push(r.a.createElement(N.a,{item:!0,key:ee()},r.a.createElement(p.a,null,"Your Clues:"))),e.round.clues.forEach((function(e,t){n.push(r.a.createElement(N.a,{item:!0,key:ee()},r.a.createElement(p.a,null,t+1,". ",e)))})),n.push(r.a.createElement(N.a,{item:!0,key:ee()},r.a.createElement(p.a,null,"Spy Clues:"))),e.round.spy_clues.forEach((function(e,t){n.push(r.a.createElement(N.a,{item:!0,key:ee()},r.a.createElement(p.a,null,t+1,". ",e)))})),n=r.a.createElement(N.a,{container:!0,direction:"column",justify:"center"},n)):n=r.a.createElement(N.a,{container:!0,justify:"center"},r.a.createElement(x.a,{justify:"center",disableShrink:!0}));else{if("clues"in e.round)n=r.a.createElement(N.a,{container:!0,justify:"center"},r.a.createElement(x.a,{justify:"center",disableShrink:!0}));else for(var c=0;c<3;++c){var o="clues"+c;n.push(r.a.createElement(N.a,{item:!0,key:o},r.a.createElement(f.a,{id:o,name:c.toString(),autoComplete:"off",label:"Clue #"+(c+1),className:t.clueField,margin:"normal",onChange:e.setClues})))}"order"in e.round&&(a=r.a.createElement(se,{order:e.round.order})),"clues"in e.round||s.push(r.a.createElement(y.a,{key:"submitc",variant:"contained",color:"primary",onClick:e.submitClues},"Submit Clues"))}return r.a.createElement(J.a,{expanded:e.expanded===e.round.number,onChange:e.handleExpansion(e.round.number)},r.a.createElement(A.a,{expandIcon:r.a.createElement(H.a,null)},r.a.createElement(p.a,{color:"textPrimary",variant:"h6"},"Round ",e.round.number+1)),r.a.createElement(L.a,null,r.a.createElement(N.a,{className:t.card,spacing:2,container:!0},a,n),s))}function ie(e){var t=[],a=function(e){return e.getHours()+":"+e.getMinutes().toString().padStart(2,"0")};e.chat.forEach((function(e,n){"system"in e?t.push(r.a.createElement(E.a,{key:e.date+e.date.getMilliseconds()+e.name+e.chat},r.a.createElement(p.a,{display:"inline"},"[",a(e.date),"]"),r.a.createElement(p.a,{display:"inline",color:"textSecondary",fontStyle:"italic"},": ",e.message))):t.push(r.a.createElement(E.a,{key:e.date+e.date.getMilliseconds()+e.name+e.chat},r.a.createElement(p.a,{display:"inline"},"[",a(e.date),"]"),r.a.createElement(p.a,{display:"inline"}," ",r.a.createElement("b",null,e.name)),r.a.createElement(p.a,{display:"inline"},": ",e.message)))}));return r.a.createElement(r.a.Fragment,null,r.a.createElement(E.a,{id:e.id,className:e.className},t),r.a.createElement(f.a,{name:"chat",fullWidth:!0,onKeyUp:function(t){t.preventDefault(),"Enter"===t.key&&(e.sendchat(t.target.value),t.target.value=null)},autoComplete:"off",label:"Press enter to send chat...",margin:"normal"}))}function ce(e){for(var t=[],a=0;a<e.clues.length;++a){for(var n=[],s=1;s<e.clues[a].length;++s)n.push(r.a.createElement(Q.a,{key:s},e.clues[a][s]?e.clues[a][s]:""));t.push(r.a.createElement(Z.a,{key:a},n))}return r.a.createElement(V.a,null,r.a.createElement(X.a,null,r.a.createElement(Z.a,null,r.a.createElement(Q.a,null,"Word 1"),r.a.createElement(Q.a,null,"Word 2"),r.a.createElement(Q.a,null,"Word 3"),r.a.createElement(Q.a,null,"Word 4"))),r.a.createElement(K.a,null,t))}function oe(e){return r.a.createElement(E.a,{hidden:e.value!==e.index},e.children)}var me=function(e){function t(e){var a;return Object(o.a)(this,t),(a=Object(u.a)(this,Object(h.a)(t).call(this,e))).state={team_a:new Set,team_b:new Set,players:new Set,me:"",host:"",state:"setup",s:null,connected:!1,message:"Select a team to join!",words:[],rounds:[],drawer:!1,clues:[],guesses:[],spy_guesses:[],score:null,expanded:!1,tab:0,all_chat:[],team_chat:[],clue_view:[],spy_clue_view:[]},a}return Object(d.a)(t,e),Object(m.a)(t,[{key:"onMessage",value:function(e){if(console.log("Got server response:"),console.log(e.data),e.data){var t=null;try{t=JSON.parse(e.data)}catch(g){return void console.log(g)}if("command"in t)switch(t.command){case"error":console.log("ERROR: "+t.msg),this.error(t.msg);break;case"player_connected":var a=new Set(this.state.players);a.add(t.player),this.setState({players:a}),1===this.state.players.size&&this.setState({me:t.player}),this.push_chat("all_chat",te(t.player+" connected."));break;case"player_disconnected":var n=new Set(this.state.players);n.delete(t.player),this.setState({players:n}),this.push_chat("all_chat",te(t.player+" disconnected."));break;case"joined_team":if(this.state.me===t.name&&this.setState({message:"Waiting for game to start..."}),"a"===t.team){var r=new Set(this.state.team_a);r.add(t.name),this.setState({team_a:r})}else{if("b"!==t.team)return void console.log("ERROR: joined_team: invalid team: "+t.team);var s=new Set(this.state.team_b);s.add(t.name),this.setState({team_b:s})}this.push_chat("all_chat",te(t.name+" joined team "+t.team.toUpperCase()));break;case"left_team":if("a"===t.team){var l=new Set(this.state.team_a);l.delete(t.name),this.setState({team_a:l})}else{if("b"!==t.team)return void console.log("ERROR: left_team: invalid team: "+t.team);var i=new Set(this.state.team_b);i.delete(t.name),this.setState({team_b:i})}this.push_chat("all_chat",te(t.name+" left team "+t.team.toUpperCase()));break;case"new_host":this.setState({host:t.player}),this.push_chat("all_chat",te(t.player+" is the new host"));break;case"words":this.setState({state:"words"}),this.setState({words:t.words.slice(0)});break;case"round":var c=this.state.rounds.slice(0);if(c[t.number]=Object(T.a)({},c[t.number],{},t),"clues"in c[t.number]&&"guesses"in c[t.number]?this.setState({rounds:c,message:"Guess the enemies order...",spy_guesses:[],expanded:t.number}):"clues"in c[t.number]?this.setState({rounds:c,message:"Match the clues with the word...",guesses:[],expanded:t.number}):this.setState({rounds:c,message:"Waiting for clues...",clues:[],expanded:t.number}),"order"in t&&"spy_order"in t){for(var o=this.state.clue_view,m=this.state.spy_clue_view,u=[],h=[],d=0;d<t.order.length;++d)u[t.order[d]]=t.clues[d],h[t.spy_order[d]]=t.spy_clues[d];o[t.number]=u,m[t.number]=h,this.setState({clue_view:o,spy_clue_view:m})}break;case"order":var p=this.state.rounds.slice(0);if(p[t.number]||(p[t.number]={}),"order"in p[t.number])break;p[t.number].order=t.order,this.setState({rounds:p,message:"Please give clues that match the order"});break;case"score":"winner"in t?this.setState({score:t,message:t.winner+" wins the game! The words are: Team A: "+t.words.team_a+" Team B:"+t.words.team_b}):"tie"in t?this.setState({score:t,message:"Game is a tie! The words are: Team A: "+t.words.team_a+" Team B:"+t.words.team_b}):this.setState({score:t});break;case"team_chat":this.push_chat("team_chat",t);break;case"all_chat":this.push_chat("all_chat",t);break;default:console.log("Unhandled: "+t)}else console.log("No command in response from server. "+e)}}},{key:"createSocket",value:function(e){var t=this,a="ws://"+document.domain+":"+window.location.port+window.location.pathname+"/ws";var n=new WebSocket(a);return n.onerror=function(e){console.log("ERROR:"),console.log(e)},n.onmessage=function(e){return t.onMessage(e)},n.onopen=function(e){return t.setState({connected:!0})},n.onclose=function(e){return t.setState({connected:!1})},n}},{key:"componentDidMount",value:function(){null===this.state.s&&this.setState({s:this.createSocket(this.props)})}},{key:"componentWillUnmount",value:function(){null===this.state.s&&(this.state.s.close(),this.setState({s:null}))}},{key:"send",value:function(e){this.state.s&&this.state.s.readyState===this.state.s.OPEN?this.state.s.send(JSON.stringify(e)):this.error("Websocket not connected! Try refreshing.")}},{key:"error",value:function(e){var t=this;this.props.enqueueSnackbar(e,{variant:"error",action:function(e){return r.a.createElement(y.a,{onClick:function(){t.props.closeSnackbar(e)}},"Dismiss ",r.a.createElement(U.a,null))}})}},{key:"join_a",value:function(){this.send({command:"join_a"})}},{key:"join_b",value:function(){this.send({command:"join_b"})}},{key:"leave_team",value:function(){this.send({command:"leave_team"})}},{key:"start_game",value:function(){this.send({command:"start_game"})}},{key:"set_field",value:function(e,t){var a=this.state[e],n={};"number"===t.target.type?a[parseInt(t.target.name)]=parseInt(t.target.value):a[parseInt(t.target.name)]=t.target.value,n[e]=a,this.setState(n)}},{key:"submit_field",value:function(e,t){var a,n=this.state[e];3===n.length?n.reduce((function(e,t){return e&&t}),!0)?this.send((a={command:e},Object(R.a)(a,e,n),Object(R.a)(a,"number",this.state.rounds.length-1),a)):this.error("Some "+e+" empty!"):this.error("Only "+n.length+" set!")}},{key:"send_chat",value:function(e,t){this.send({command:e,message:t})}},{key:"push_chat",value:function(e,t){var a=this.state[e];a.push(Object(T.a)({date:new Date},t)),this.setState(Object(R.a)({},e,a));var n=document.getElementById(e);n&&n.scrollTop>=n.scrollHeight-n.clientHeight-n.lastChild.clientHeight&&(n.scrollTop=n.scrollHeight)}},{key:"getClueGiver",value:function(){return"words"!==this.state.state?null:0===this.state.rounds.length?null:this.state.rounds[this.state.rounds.length-1].clue_giver}},{key:"getSpyClueGiver",value:function(){return"words"!==this.state.state?null:0===this.state.rounds.length?null:this.state.rounds[this.state.rounds.length-1].spy_clue_giver}},{key:"toggleDrawer",value:function(){this.setState({drawer:!this.state.drawer})}},{key:"handleExpansion",value:function(e){var t=this;return function(a,n){t.setState({expanded:!!n&&e})}}},{key:"handleTabChange",value:function(e,t){this.setState({tab:t})}},{key:"render",value:function(){var e=this.props.classes,t=[],a=[],n=this.state.host,s=this.state.me,l=this.getClueGiver(),i=this.getSpyClueGiver();[[this.state.team_a,t],[this.state.team_b,a]].forEach((function(t){t[0].forEach((function(a){var n=a===l||a===i?"CG":0,c=s===a?"textPrimary":"textSecondary";t[1].push(r.a.createElement(N.a,{xs:!0,item:!0,key:a},r.a.createElement(D.a,{badgeContent:n,color:"secondary",className:e.padding},r.a.createElement(p.a,{component:"h5",align:"center",color:c,key:a},a)),r.a.createElement(W.a,null)))}))}));var c=[],o=[];"setup"===this.state.state?this.state.team_a.has(this.state.me)||this.state.team_b.has(this.state.me)?(c.push(r.a.createElement(ne,{key:"leave",onClick:this.leave_team.bind(this)})),this.state.me===n&&c.push(r.a.createElement(re,{key:"start",onClick:this.start_game.bind(this)}))):c=r.a.createElement(ae,{key:"join",team_a:this.join_a.bind(this),team_b:this.join_b.bind(this)}):this.state.words.length>0&&this.state.words.forEach((function(t,a){o.push(r.a.createElement(N.a,{item:!0,key:t},r.a.createElement(D.a,{badgeContent:a+1,color:"secondary"},r.a.createElement(b.a,{className:e.card},r.a.createElement(p.a,{component:"h1",variant:"h3",align:"center",color:"textPrimary",gutterBottom:!0},t)))))}));var m=[],u=this;this.state.rounds.forEach((function(e){m.push(r.a.createElement(N.a,{item:!0,key:e.number},r.a.createElement(le,{round:e,me:s,expanded:u.state.expanded,handleExpansion:u.handleExpansion.bind(u),setClues:u.set_field.bind(u,"clues"),submitClues:u.submit_field.bind(u,"clues"),setGuesses:u.set_field.bind(u,"guesses"),submitGuesses:u.submit_field.bind(u,"guesses"),setSpyGuesses:u.set_field.bind(u,"spy_guesses"),submitSpyGuesses:u.submit_field.bind(u,"spy_guesses")})))}));var h=[];return this.state.score&&(h=r.a.createElement(E.a,null,r.a.createElement(N.a,{container:!0,justify:"center",spacing:1},r.a.createElement(N.a,{item:!0},r.a.createElement(b.a,{className:e.padding},r.a.createElement(p.a,{variant:"h6"},"Team A"),r.a.createElement(p.a,null,"Intercepts: ",this.state.score.team_a.intercepts),r.a.createElement(p.a,null,"Miscommunications: ",this.state.score.team_a.miscommunications))),r.a.createElement(N.a,{item:!0},r.a.createElement(b.a,{className:e.padding},r.a.createElement(p.a,{variant:"h6"},"Team B"),r.a.createElement(p.a,null,"Intercepts: ",this.state.score.team_b.intercepts),r.a.createElement(p.a,null,"Miscommunications: ",this.state.score.team_b.miscommunications)))))),r.a.createElement(r.a.Fragment,null,r.a.createElement(k.a,null),r.a.createElement(_.a,{position:"relative"},r.a.createElement(S.a,null,r.a.createElement(I.a,{className:e.icon,onClick:this.toggleDrawer.bind(this)}),r.a.createElement(p.a,{variant:"h6",color:"inherit",noWrap:!0},"Decrypto"))),r.a.createElement(M.a,{open:this.state.drawer,onClose:this.toggleDrawer.bind(this)},r.a.createElement(E.a,null,r.a.createElement(b.a,null,r.a.createElement(p.a,{component:"h3",variant:"h5",align:"center",color:"textPrimary",className:e.teamName},"Team A"),t),r.a.createElement(b.a,null,r.a.createElement(p.a,{component:"h3",variant:"h5",align:"center",color:"textPrimary",className:e.teamName},"Team B"),a))),r.a.createElement(w.a,{open:!this.state.connected},r.a.createElement(C.a,null,r.a.createElement(N.a,{container:!0,justify:"center"},r.a.createElement(x.a,{justify:"center",disableShrink:!0})),r.a.createElement(p.a,{component:"h2",align:"center"},"Waiting for WebSocket connection...")),r.a.createElement(j.a,null,r.a.createElement(y.a,{onClick:function(e){return window.location.reload(!1)},color:"primary"},"Refresh"))),r.a.createElement(E.a,{className:e.heroContent},r.a.createElement(E.a,null,h),r.a.createElement(E.a,{maxWidth:"sm"},r.a.createElement(p.a,{component:"h1",variant:"h4",align:"center",color:"textPrimary",gutterBottom:!0},this.state.message)),r.a.createElement(E.a,{className:e.heroButtons},c),r.a.createElement(E.a,{className:e.icon},r.a.createElement(N.a,{container:!0,spacing:2,justify:"center"},o))),r.a.createElement(E.a,null,m),r.a.createElement(E.a,{className:e.bottomBar},r.a.createElement(_.a,{elevation:0,className:e.appBar,position:"static"},r.a.createElement(z.a,{value:this.state.tab,onChange:this.handleTabChange.bind(this),scrollButtons:"on",variant:"scrollable"},r.a.createElement(q.a,{label:"All Chat",index:0}),r.a.createElement(q.a,{label:"Team Chat",index:1}),r.a.createElement(q.a,{label:"Clue View",index:2}),r.a.createElement(q.a,{label:"Spy Clue View",index:3}))),r.a.createElement(oe,{value:this.state.tab,index:0},r.a.createElement(ie,{id:"all_chat",className:e.chatBox,sendchat:this.send_chat.bind(this,"all_chat"),chat:this.state.all_chat})),r.a.createElement(oe,{value:this.state.tab,index:1},r.a.createElement(ie,{id:"team_chat",className:e.chatBox,sendchat:this.send_chat.bind(this,"team_chat"),chat:this.state.team_chat})),r.a.createElement(oe,{value:this.state.tab,index:2},r.a.createElement(ce,{clues:this.state.clue_view})),r.a.createElement(oe,{value:this.state.tab,index:3},r.a.createElement(ce,{clues:this.state.spy_clue_view}))))}}]),t}(r.a.Component),ue=Object(v.withSnackbar)(Object(g.a)($)(me));l.a.render(r.a.createElement((function(){return r.a.createElement(v.SnackbarProvider,{maxSnack:3},r.a.createElement("main",null,r.a.createElement(i.a,null,r.a.createElement(c.a,{exact:!0,path:"/",component:B}),r.a.createElement(c.a,{path:"/game",component:ue}))))}),null),document.getElementById("root"))}},[[107,1,2]]]);
//# sourceMappingURL=main.702ac06b.chunk.js.map