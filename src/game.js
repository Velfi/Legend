function Game() {}

Game.prototype.create = function() {
  platforms = game.add.group();
  platforms.enableBody = true;
  ground = platforms.create(0, game.world.height - 64, 'ground');
  ground.scale.setTo(2, 2);
  ground.body.immovable = true;
  ledge = platforms.create(400, 400, 'ground');
  ledge.body.immovable = true;
  ledge = platforms.create(-150, 250, 'ground');
  ledge.body.immovable = true;
  player = game.add.sprite(32, game.world.height - 150, 'dude');
  game.physics.arcade.enable(player);
  player.body.bounce.y = 0.2;
  player.body.gravity.y = 300;
  player.body.collideWorldBounds = true;
  player.animations.add('left', [0, 1, 2, 3], 10, true);
  player.animations.add('right', [5, 6, 7, 8], 10, true);
  this.game.stage.backgroundColor = 0x4488cc;
};

Game.prototype.update = function() {
  game.physics.arcade.collide(player, platforms);
  player.body.velocity.x = 0;
  if (cursors.left.isDown) {
    player.body.velocity.x = -150;
    player.animations.play('left');
  } else if (cursors.right.isDown) {
    player.body.velocity.x = 150;
    player.animations.play('right');
  } else {
    player.animations.stop();
    player.frame = 4;
  }
  if (cursors.up.isDown && player.body.touching.down) {
    player.body.velocity.y = -350;
  }
}


Game.prototype.onInputDown = function() {};

module.exports = Game;
