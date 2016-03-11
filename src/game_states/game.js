var Player = require('/home/zelda/Documents/phaser_game/src/player.js');

function Game() {}

Game.prototype.create = function() {
  player = new Player(200, 200);
  player.create();
  map = game.add.tilemap('map', 64, 64);
  map.addTilesetImage('tiles');
  layer = map.createLayer(0);
  game.physics.arcade.enable(layer);
  layer.setCollision(0);
  layer.resizeWorld();
  this.game.stage.backgroundColor = 0x4488cc;
};

Game.prototype.update = function() {
  player.update();
}


Game.prototype.onInputDown = function() {};

module.exports = Game;

// Level.prototype = {
// initLevel: function(tilemapKey, tilesetImage, tilesetImageKey) {
//   level = this;
//
//   game.physics.arcade.setBoundsToWorld();
//
//   this.map = game.add.tilemap(tilemapKey);
//   this.map.addTilesetImage(tilesetImage, tilesetImageKey);
// },
// restart: function() {
//   this.resetCamera();
//   this.enemyGroup.forEach(function(enemy) {
//     enemy.kill();
//     if ('killProjectiles' in enemy.parentEntity) {
//       enemy.parentEntity.killProjectiles();
//     }
//   }, this);
//   this.enemyGroup.destroy();
//   if ('tearDownLevelComponents' in this) {
//     this.tearDownLevelComponents();
//   }
//   if ('buildLevelComponents' in this) {
//     this.buildLevelComponents();
//   }
//   this.enemyGroup = game.add.group();
//   this.createEnemies();
//   player.create();
// },
// render: function() {
//   if (window.debugging == true) {
//     game.debug.body(player.sprite);
//     this.enemyGroup.forEach(function(enemy) {
//       game.debug.body(enemy);
//       if (enemy.parentEntity.projectiles) {
//         enemy.parentEntity.projectiles.forEach(function(projectile) {
//           game.debug.body(projectile);
//         }, this);
//       }
//     }, this);
//   }
// },
// resetCamera: function() {
//   game.camera.x = this.startingCameraPosX;
//   game.camera.y = this.startingCameraPosY;
// },
// };
// $.extend(Level.prototype, FadableState.prototype);
