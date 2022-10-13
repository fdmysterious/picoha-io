from setuptools import setup, find_packages
from setuptools.command.install import install

VERSION = '0.0.1'
DESCRIPTION = 'Panduza Python MetaDrivers to manage Picoha version'
LONG_DESCRIPTION = ''

class CustomInstallCommand(install):
    def run(self):
        install.run(self)

# Setting up
setup(
    name="panduza_class_picoha",
    version=VERSION,
    author="Panduza Team",
    author_email="panduza.team@gmail.com",
    description=DESCRIPTION,
    long_description=LONG_DESCRIPTION,
    packages=find_packages(),
    cmdclass={'install': CustomInstallCommand},

    install_requires=[
        # 'panduza_platform',
        'pyudev',
        'pyserial'
        ],

    classifiers=[
        "Development Status :: 3 - Alpha",
        "Programming Language :: Python :: 2",
        "Programming Language :: Python :: 3",
    ]
)

