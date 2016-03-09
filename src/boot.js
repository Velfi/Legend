function Boot() {}

Boot.prototype.preload = function() {
  this.load.image('preloader', 'assets/preloader.gif');
};

Boot.prototype.create = function() {
  this.game.input.maxPointers = 1;
  // this.scale.scaleMode = Phaser.ScaleManager.SHOW_ALL;
  this.scale.pageAlignVertically = true;
  this.scale.pageAlignHorizontally = true;
  this.game.state.start('preloader');
};

module.exports = Boot;
