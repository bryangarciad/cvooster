mod migrations;

use migrations::migration001::migration;

fn main() {
    migration();
}