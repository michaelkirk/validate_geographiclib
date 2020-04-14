#!/usr/bin/env python3

import fileinput
import sys

from geographiclib.geodesic import Geodesic
from geographiclib.geodesiccapability import GeodesicCapability

def eprintln(str):
    sys.stderr.write(str + "\n")

class Runner(object):
    def __init__(self, args):
        self.geodesic = Geodesic.WGS84
        self.is_inverse = args.is_inverse
        self.is_full_output = args.is_full_output

        # TODO: we don't do anything with precision, it's just there for API
        # compatibility with the validation tool.
        self.precision = args.precision

    def run(self):
        for line in fileinput.input('-'):
            # eprintln("line: %s" % line)
            fields = [float(x) for x in line.split(" ")]

            outmask = GeodesicCapability.ALL

            if self.is_inverse:
                # gunzip -c GeodTest.dat.gz | cut -d' ' -f1,2,4,5 | ./GeodSolve -i
                assert(len(fields) == 4)
                lat1 = fields[0]
                lon1 = fields[1]
                lat2 = fields[2]
                lon2 = fields[3]

                result = self.geodesic.Inverse(lat1, lon1, lat2, lon2, outmask=outmask)
                if self.is_full_output:
                    # TODO - we're currently omitting several fields, and only outputting what's 
                    # necessary to pass the validation tool
                    output_fields = [
                        lat1,
                        lon1,
                        result["azi1"],
                        lat2,
                        lon2,
                        result["azi2"],
                        result["s12"],
                        result["a12"],
                        result["m12"]
                    ]
                else:
                    output_fields = [
                        result["azi1"],
                        result["azi2"],
                        result["s12"],
                    ]
            else:
                # gunzip -c GeodTest.dat.gz | cut -d' ' -f1,2,3,7 | ./GeodSolve
                assert(len(fields) == 4)
                lat1 = fields[0]
                lon1 = fields[1]
                azi1 = fields[2]
                s12 = fields[3]

                result = self.geodesic.Direct(lat1, lon1, azi1, s12, outmask=outmask)

                if self.is_full_output:
                    # TODO - we're currently omitting several fields, and only outputting what's 
                    # necessary to pass the validation tool
                    output_fields = [
                        lat1,
                        lon1,
                        azi1,
                        result["lat2"],
                        result["lon2"],
                        result["azi2"],
                        s12,
                        result["a12"],
                        result["m12"]
                    ]
                else:
                    output_fields = [
                        result["lat2"],
                        result["lon2"],
                        result["azi2"],
                    ]

            output = [str(f) for f in output_fields]
            print(" ".join(output), flush=True)

if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser(description='Process some integers.')
    parser.add_argument('-i', dest='is_inverse', default=False, action='store_true', help='perform an inverse geodesic calculation')
    parser.add_argument('-f', dest='is_full_output', default=False, action='store_true', help='full output')
    parser.add_argument('-p', dest='precision', default=3, help='full output')
    args = parser.parse_args()

    try:
        Runner(args).run()
        # flush output here to force SIGPIPE to be triggered
        # while inside this try block.
        sys.stdout.flush()
    except BrokenPipeError:
        import os
        # Python flushes standard streams on exit; redirect remaining output
        # to devnull to avoid another BrokenPipeError at shutdown
        devnull = os.open(os.devnull, os.O_WRONLY)
        os.dup2(devnull, sys.stdout.fileno())
        sys.exit(1)  # Python exits with error code 1 on EPIPE

