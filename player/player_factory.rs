pub fn spawn_controllable<F,S>(f:F)->(impl Controllable,impl Controller) where F:Fn()->(Player<S>,Sender);
