import subprocess

import glob

sources = glob.glob('./src/*.md')
srcstring = " ".join(sources)

sass_command = "sass sass/styles.scss style.css".split()

pandoc_command = f"pandoc -s {srcstring} --mathml --css style.css -o index.html".split(" ")
print(pandoc_command)


def build_style():
    print("rebuilding style...")
    subprocess.Popen(sass_command,shell=True).wait()
    print('done.')

def build_pandoc():
    print("rebuilding doc...")
    subprocess.Popen(pandoc_command,shell=True).wait()
    print("done.")

def build():
    build_style()
    build_pandoc()



import argparse
parser = argparse.ArgumentParser()
parser.add_argument("--build")
args = parser.parse_args()



if args.build:
    build()
    exit()
else:
    from watchgod import awatch
    import asyncio
    
    async def main():
        async for change in awatch("./src"):
            build_pandoc()
    
    asyncio.run(main())
