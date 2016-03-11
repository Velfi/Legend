function Preloader() {
  this.asset = null;
  this.ready = false;
}
Preloader.prototype = {
  preload: function() {
    this.asset = this.add.sprite(this.game.width * 0.5 - 110, this.game.height * 0.5 - 10, 'preloader');
    this.load.setPreloadSprite(this.asset);
    this.loadResources();
    this.ready = true;
  },
  loadResources: function() {
    game.load.image('back_trees', 'assets/start_screen/parallax-forest-back-trees1.png');
    game.load.image('middle_trees', 'assets/start_screen/parallax-forest-middle-trees1.png');
    game.load.image('front_trees', 'assets/start_screen/parallax-forest-front-trees1.png');
    game.load.image('lights_trees', 'assets/start_screen/parallax-forest-lights1.png');
    game.load.tilemap('map', 'assets/tilemaps/simple.json', null, Phaser.Tilemap.TILED_JSON);
    game.load.image('simple', 'assets/tilemaps/simple.png');
    game.load.spritesheet('richter', 'assets/richter.png', 30, 44);
    game.load.spritesheet('enemies_sheet', 'assets/enemies/enemies.png', 32, 64);
    game.load.image('startMenu', 'assets/start_screen/start_screen.png');
  },
  create: function() {},
  update: function() {
    if (this.ready) {
      // this.game.state.start('startScreen');
      this.game.state.start('game');
    }
  }
}

module.exports = Preloader;
