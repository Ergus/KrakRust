# Copyright (C) 2024  Jimmy Aguilar Mena

# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.

# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.

# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <http://www.gnu.org/licenses/>.

import pykraken as pyk
import json

def main():
    try:
        res = await pyk.get_info("pair", "BTCEUR")

        json_formatted_str = json.dumps(res, indent=2)
        print(json_formatted_str)

    except Exception as e:
        print(f"Error occurred: {e}")

if __name__ == "__main__":
    main()
