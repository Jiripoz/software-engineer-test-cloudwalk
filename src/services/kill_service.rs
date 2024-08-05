pub trait KillService {
    fn new(killer: String, victim: String, weapon: String) -> Self;
}
