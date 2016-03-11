var Player = require('/home/zelda/Documents/legend/src/player.js');
var Zombie = require('/home/zelda/Documents/legend/src/enemies/zombie.js');

function Game() {}

Game.prototype.create = function() {
  // game.physics.startSystem(Phaser.Physics.ARCADE);
  game.stage.backgroundColor = 0x224466;
  game.stage.smoothed = false;
  map = game.add.tilemap('map');
  map.addTilesetImage('simple');
  map.setCollisionBetween(1, 16, true, 'center_stage');
  center_stage = map.createLayer('center_stage');
  back_stage = map.createLayer('back_stage');
  player = new Player(64, 600);
  player.create();
  enemy = new Zombie(600, 600);
  enemy.create();
  front_stage = map.createLayer('front_stage');
  center_stage.resizeWorld();
  game.camera.follow(player.sprite, Phaser.Camera.FOLLOW_PLATFORMER);
};

Game.prototype.update = function() {
  game.physics.arcade.collide(player.sprite, center_stage);
  game.physics.arcade.collide(enemy.sprite, center_stage);
  // game.physics.arcade.collide(enemy.sprite, player.sprite);
  player.update();
  enemy.update();
}

Game.prototype.onInputDown = function() {};

Game.prototype.render = function() {
  game.debug.body(player.sprite);
  game.debug.body(enemy.sprite);

  game.debug.bodyInfo(player.sprite, 250, 50);
}

module.exports = Game;
