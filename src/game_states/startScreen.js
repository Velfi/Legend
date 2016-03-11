function StartScreen() {}

StartScreen.prototype = {
  create: function() {
    //here we add two tile sprites with parameters(locationX, locationY, width, height, key)
    this.bg_bt = game.add.tileSprite(0, 0, game.world.width, game.world.height, 'back_trees');
    this.bg_bt.smoothed = false;
    this.bg_mt = game.add.tileSprite(0, 0, game.world.width, game.world.height, 'middle_trees');
    this.bg_mt.smoothed = false;
    this.bg_ft = game.add.tileSprite(0, 0, game.world.width, game.world.height, 'front_trees');
    this.bg_ft.smoothed = false;
    this.bg_lt = game.add.tileSprite(0, 0, game.world.width, game.world.height, 'lights_trees');
    this.bg_lt.smoothed = false;
    game.add.sprite(0, 0, 'startMenu');
    this.input.onDown.add(this.onInputDown, this);
  },
  update: function() {
    this.bg_bt.tilePosition.x += .3;
    this.bg_mt.tilePosition.x += .6;
    this.bg_ft.tilePosition.x += .9;
    this.bg_lt.tilePosition.x += 1.2;
  },
  onInputDown: function() {
    this.game.state.start('game');
  }
}

module.exports = StartScreen;
