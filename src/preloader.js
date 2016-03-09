function Preloader() {
  this.asset = null;
  this.ready = false;
}

Preloader.prototype.preload = function() {
  this.asset = this.add.sprite(this.game.width * 0.5 - 110, this.game.height * 0.5 - 10, 'preloader');
  this.load.setPreloadSprite(this.asset);
  this.load.onLoadComplete.addOnce(this.onLoadComplete, this);
  this.loadResources();
  this.ready = true;
};

Preloader.prototype.loadResources = function() {
  this.game.load.image('sky', 'assets/sky.png');
  this.game.load.image('ground', 'assets/platform.png');
  this.game.load.image('star', 'assets/star.png');
  this.game.load.spritesheet('dude', 'assets/dude.png', 32, 48);
  this.game.load.spritesheet('tileset', 'assets/tileset.png', 32, 32);
};

Preloader.prototype.create = function() {
  this.game.physics.startSystem(Phaser.Physics.ARCADE);
  cursors = game.input.keyboard.createCursorKeys();
};

Preloader.prototype.update = function() {
  // if (!!this.ready) {
  this.game.state.start('menu');
  // }
};

Preloader.prototype.onLoadComplete = function() {
  // this.ready = true;
};

module.exports = Preloader;
