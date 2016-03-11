function Zombie(xSpawnPos, ySpawnPos) {
  this.xSpawnPos = xSpawnPos;
  this.ySpawnPos = ySpawnPos;
}

Zombie.prototype = {
  create: function() {
    this.sprite = game.add.sprite(this.xSpawnPos, this.ySpawnPos, 'enemies_sheet');
    this.sprite.scale.set(2);
    this.initializeEnemyPhysics();
    this.initializeEnemyAnimations();
    // this.initializeEnemyAudio();
    // this.sprite.body.setSize(PLAYER_WIDTH, PLAYER_HEIGHT, PLAYER_X_OFFSET, PLAYER_Y_OFFSET);
    this.sprite.body.setSize(32, 40, 0, 45); // make bounding box more forgiving
  },
  initializeEnemyPhysics: function() {
    // this.enemyAcceleration = 1200; // pixels/second/second
    this.enemyDrag = 2800; // pixels/second
    this.enemyGravity = 1200; // pixels/second/second
    // this.enemyJumpSpeed = -600;
    this.enemyMaxSpeed = 100; // pixels/second

    game.physics.enable(this.sprite);
    this.sprite.body.bounce.y = 0.2;
    this.sprite.body.collideWorldBounds = true;
    this.sprite.body.drag.setTo(this.enemyDrag, 0);
    this.sprite.body.gravity.y = this.enemyGravity;
    this.sprite.body.maxVelocity.setTo(this.enemyMaxSpeed, this.enemyMaxSpeed * 10);
  },
  initializeEnemyAnimations: function() {
    this.sprite.animations.add('left', [10, 11, 12], 10, true);
    this.sprite.animations.add('right', [19, 20, 21], 10, true);
  },
  update: function() {
    if(player.sprite.x - enemy.sprite.x < 48 && player.sprite.x - enemy.sprite.x > -48 ){
      enemy.sprite.body.acceleration.x = 0;
    } else if (player.sprite.x < enemy.sprite.x) {
      enemy.sprite.body.acceleration.x = -150;
    } else {
      enemy.sprite.body.acceleration.x = 150;
    }
  }
};

module.exports = Zombie
