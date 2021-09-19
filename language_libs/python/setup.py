from setuptools import find_packages, setup

setup(
    name='faassa',
    include_package_data=True,
    packages=find_packages(),
    version='0.1.8',
    description='Faas-storage-agent apis',
    author='why',
    license='MIT',
    install_requires=['requests>=2.2.0', 'grpcio>=1.30.0', 'grpcio_tools>=1.30.0'],
    setup_requires=['pytest-runner'],
    tests_require=['pytest'],
    test_suite='tests',
)