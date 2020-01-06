// CBC State Machine
//
// State : Type
// State = OpenS + ClosedS
// \exists TF: Adj(s) for s: State
// Closed: Adj(OpenS)
// Open: Adj(ClosedS)
// trans: TF -> Adj(s)

trait Trans<S> {}

struct Open;
struct Closed;

trait OpenAdj {}
trait ClosedAdj {}

impl ClosedAdj for Open {}
impl OpenAdj for Closed {}

impl<N> Trans<N> for Open where N: OpenAdj {}

impl<N> Trans<N> for Closed where N: ClosedAdj {}