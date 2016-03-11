game = new Phaser.Game(1280, 720, Phaser.AUTO, 'Legend-game');
game.state.add('boot', require('./game_states/boot'));
game.state.add('preloader', require('./game_states/preloader'));
game.state.add('startScreen', require('./game_states/startScreen'));
game.state.add('game', require('./game_states/game'));
game.state.start('boot');
