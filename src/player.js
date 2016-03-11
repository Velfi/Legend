function Player(xSpawnPos, ySpawnPos) {
  this.xSpawnPos = xSpawnPos;
  this.ySpawnPos = ySpawnPos;
}

Player.prototype = {
  create: function() {
    this.facing = 'right';
    this.sprite = game.add.sprite(this.xSpawnPos, this.ySpawnPos, 'richter');
    this.sprite.scale.set(2);
    this.smoothed = false;
    // game.camera.follow(player.sprite);

    this.initializePlayerPhysics();
    this.initializePlayerAnimations();
    this.initializePlayerControls();
    // this.initializePlayerAudio();
    // this.sprite.body.setSize(PLAYER_WIDTH, PLAYER_HEIGHT, PLAYER_X_OFFSET, PLAYER_Y_OFFSET);
  },
  initializePlayerPhysics: function() {
    this.playerAcceleration = 1200; // pixels/second/second
    this.playerDrag = 2800; // pixels/second
    this.playerGravity = 1200; // pixels/second/second
    this.playerJumpSpeed = -600;
    this.playerMaxSpeed = 300; // pixels/second

    game.physics.arcade.enable(this.sprite);
    this.sprite.body.bounce.y = 0.1;
    this.sprite.body.collideWorldBounds = true;
    this.sprite.body.drag.setTo(this.playerDrag, 0);
    this.sprite.body.gravity.y = this.playerGravity;
    this.sprite.body.maxVelocity.setTo(this.playerMaxSpeed, this.playerMaxSpeed * 10);
  },
  initializePlayerControls: function() {
    buttonJump = game.input.keyboard.addKey(Phaser.Keyboard.UP);
    buttonDuck = game.input.keyboard.addKey(Phaser.Keyboard.DOWN);
    buttonMoveLeft = game.input.keyboard.addKey(Phaser.Keyboard.LEFT);
    buttonMoveRight = game.input.keyboard.addKey(Phaser.Keyboard.RIGHT);
    // this.cursors = game.input.keyboard.createCursorKeys();
    // this.jumpButton = game.input.keyboard.addKey(Phaser.Keyboard.SPACEBAR);
  },
  initializePlayerAnimations: function() {
    this.sprite.animations.add('left', [0, 1, 2, 3, 4, 5, 6, 7], 10, true);
    this.sprite.animations.add('right', [8, 9, 10, 11, 12, 13, 14, 15], 10, true);
    this.sprite.animations.add('jump', [19, 20, 21, 22], 10, true);
  },
  update: function() {
    this.updateMovement();
  },
  updateMovement: function() {
    this.sprite.body.acceleration.x = 0;
    if (buttonMoveLeft.isDown) {
      this.sprite.body.acceleration.x = -this.playerAcceleration;
      this.sprite.animations.play('left');
      this.player_facing = 'left';
    } else if (buttonMoveRight.isDown) {
      this.sprite.body.acceleration.x = this.playerAcceleration;
      this.sprite.animations.play('right');
      this.player_facing = 'right';
    } else {
      if (this.player_facing === 'right') {
        this.sprite.animations.stop();
        this.sprite.frame = 16;
      } else {
        this.sprite.animations.stop();
        this.sprite.frame = 17;
      }
    }
    if (buttonJump.isDown && this.sprite.body.blocked.down) {
      this.sprite.body.velocity.y = this.playerJumpSpeed;
    }
  }
}

module.exports = Player
