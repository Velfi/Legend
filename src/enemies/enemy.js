function Enemy(xSpawnPos, ySpawnPos) {
  this.xSpawnPos = xSpawnPos;
  this.ySpawnPos = ySpawnPos;
}

Enemy.prototype = {
  create: function() {
    this.sprite = game.add.sprite(this.xSpawnPos, this.ySpawnPos, 'enemies_sheet');
    // this.sprite.scale.set(2);
    this.initializeEnemyPhysics();
    // this.initializeEnemyAnimations();
    // this.initializeEnemyAudio();
    // this.sprite.body.setSize(PLAYER_WIDTH, PLAYER_HEIGHT, PLAYER_X_OFFSET, PLAYER_Y_OFFSET);
    // this.sprite.body.setSize(14, 42, 16, 5); // make bounding box more forgiving
  },
  initializeEnemyPhysics: function() {
    this.enemyAcceleration = 1200; // pixels/second/second
    this.enemyDrag = 2800; // pixels/second
    this.enemyGravity = 1200; // pixels/second/second
    this.enemyJumpSpeed = -600;
    this.enemyMaxSpeed = 300; // pixels/second

    game.physics.enable(this.sprite);
    this.sprite.body.bounce.y = 0.1;
    this.sprite.body.collideWorldBounds = true;
    this.sprite.body.drag.setTo(this.enemyDrag, 0);
    this.sprite.body.gravity.y = this.enemyGravity;
    this.sprite.body.maxVelocity.setTo(this.enemyMaxSpeed, this.enemyMaxSpeed * 10);
  },
  initializeEnemyAnimations: function() {
  },
  update: function() {
  }
};

module.exports = Enemy
