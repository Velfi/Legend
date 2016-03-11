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
    game.load.tilemap('map', 'assets/tilemaps/level1.csv', null, Phaser.Tilemap.CSV);
    game.load.image('tiles', 'assets/tilemaps/tiles.png');
    game.load.spritesheet('richter', 'assets/richter.png', 30, 44);
    game.load.image('startMenu', 'assets/start_screen/start_screen.png');
  },
  create: function() {game.physics.startSystem(Phaser.Physics.ARCADE);},
  update: function() {
    if (this.ready) {
      // this.game.state.start('startScreen');
      this.game.state.start('game');
    }
  },
}

module.exports = Preloader;
