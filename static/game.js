team_a = new Set();
team_b = new Set();
players = new Set();
me = '';
host = '';
state = 'setup';

s = new WebSocket('ws://' + document.domain + ':' + location.port + '/game_ws');

s.onerror = function (e) {
  console.log("ERROR:");
  console.log(e);
}

s.onmessage = function (msg) {
  console.log('Got server response:');
  console.log(msg.data);
  if (!msg.data) return;
  try {
    d = JSON.parse(msg.data);
  } catch (e) {
    console.log(e);
    return;
  }
  if (!('command' in d)) {
    console.log('No command in response from server. ' + msg);
    return;
  }

  switch (d['command']) {
    case 'join_game': {
      window.location.href = d['game']
    } break;
    case 'player_connected': {
      players.add(d['player']);
      on_player_connected(d['player']);
    } break;
    case 'player_disconnected': {
      players.remove(d['player']);
      on_player_disconnected(d['player']);
    } break;
    case 'joined_team': {
      var team = null;
      console.log(d);
      if (d['team'] === 'a') {
        team_a.add(d['name']);
        team = team_a;
      } else if (d['team'] === 'b') {
        team_b.add(d['name']); 
        team = team_b; 
      } else {
        console.log('ERROR: joined_team: invalid team: ' + d['team']);
        return;
      }
      on_player_joined_team(d['name'], team);
    } break;
    case 'left_team': {
      var team = null;
      if (d['team'] == 'a') {
        team_a.add(d['name']);
        team = team_a;
      } else if (d['team'] == 'b') { 
        team_b.add(d['name']); team = team_b; 
      } else {
        console.log('ERROR: left_team: invalid team: ' + d['team']);
        return;
      }
      on_player_left_team(d['name'], team);
    } break;
    case 'new_host': {
      on_new_host(d['player']);
    } break;
  }
}

function on_player_connected(player) {
  if (players.size == 1) {
    me = player;
  }
}

function on_player_disconnected(player) {
}

function on_player_joined_team(player, team) {
  console.log(player);
  console.log(team);
  var container = team === team_a ? document.getElementById('team_a') : document.getElementById('team_b');
  var child = document.createElement('p');
  child.id = player;
  child.innerHTML = player;
  if (player == host) {
    child.className = 'host';
  }
  container.appendChild(child);
  if (player == me) {
    show_team_select(false);
  }
}

function on_player_left_team(player, team) {
  var container = team === team_a ? document.getElementById('team_a') : document.getElementById('team_b');
  for (var i = 0; i < container.children.length; ++i) {
    if (container.children[i].id == player) {
      container.removeChild(container.children[i]);
      if (player == me) {
        show_team_select(true);
      }
      return;
    }
  }
}

function on_new_host(h) {
  if (host) {
    // clean up old host
  }
  host = h;
  set_host();
}

function join_a() {
  s.send(JSON.stringify({ 'command': 'join_a' }));
}

function join_b() {
  s.send(JSON.stringify({ 'command': 'join_b' }));
}

function leave_team() {
  s.send(JSON.stringify({ 'command': 'leave_team' }));
}

function start_game() {
  s.send(JSON.stringify({ 'command': 'start_game'}));
}

function show_team_select(show) {
  if (state != 'setup') {
    return;
  }  
  if (show) {
    $('#teamselect').show();
    $('#leaveteam').hide();
  } else {
    $('#teamselect').hide();
    $('#leaveteam').show();
  }
}

function set_host() {
  [team_a, team_b].forEach(function(team) {
    if (!team.has(host)) return;
    var container = team === team_a ? document.getElementById('team_a') : document.getElementById('team_b');
    for (var i = 0; i < container.children.length; ++i) {
      if (container.children[i].id == host) {
        container.children[i].className = 'host';
        return;
      }
    }
  });
  if (host == me) {
    $('#startgame').show();
  }
}
